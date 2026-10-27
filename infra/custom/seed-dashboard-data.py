#!/usr/bin/env python3
"""Seed the dashboard with test routes and clickstream stats.

Logs in as the dev Keycloak test user, creates (or reuses) a workspace and a
handful of routes under the shared domain via the real management-api, then
(re)generates ClickHouse clickstream data for those routes spanning the last
N days - so the dashboard charts have something to show.

Safe to re-run: routes/workspace are reused by name/link, and this user's
previous clickstream rows are deleted before inserting a fresh batch, so
running it again just refreshes the data instead of piling up duplicates.

Usage:
    python3 seed-dashboard-data.py
    python3 seed-dashboard-data.py --days 30
"""

import argparse
import datetime
import json
import random
import sys
import time
import urllib.error
import urllib.parse
import urllib.request


def http_json(method, url, token=None, body=None):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=method)
    req.add_header("Content-Type", "application/json")
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            raw = resp.read()
            return json.loads(raw) if raw else None
    except urllib.error.HTTPError as e:
        body_text = e.read().decode(errors="replace")
        raise RuntimeError(f"{method} {url} -> HTTP {e.code}: {body_text}") from None


def clickhouse(args, query, data=None):
    params = {
        "database": args.ch_database,
        "user": args.ch_user,
        "password": args.ch_password,
    }
    if query:
        params["query"] = query
    url = f"{args.ch_url}/?{urllib.parse.urlencode(params)}"
    req = urllib.request.Request(url, data=data, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            return resp.read()
    except urllib.error.HTTPError as e:
        raise RuntimeError(f"ClickHouse error: {e.read().decode(errors='replace')}") from None


def get_token(args):
    url = f"{args.keycloak_url}/realms/{args.realm}/protocol/openid-connect/token"
    form = urllib.parse.urlencode({
        "grant_type": "password",
        "client_id": args.client_id,
        "client_secret": args.client_secret,
        "username": args.username,
        "password": args.password,
        "scope": "openid",
    }).encode()
    req = urllib.request.Request(url, data=form, method="POST")
    req.add_header("Content-Type", "application/x-www-form-urlencoded")
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return json.loads(resp.read())["access_token"]
    except urllib.error.HTTPError as e:
        raise RuntimeError(
            f"Keycloak login failed (HTTP {e.code}): {e.read().decode(errors='replace')}\n"
            f"Check --keycloak-url/--realm/--client-id/--client-secret/--username/--password."
        ) from None


def decode_jwt_sub(token):
    payload = token.split(".")[1]
    payload += "=" * (-len(payload) % 4)
    import base64
    return json.loads(base64.urlsafe_b64decode(payload))["sub"]


def ensure_workspace(args, token):
    workspaces = http_json("GET", f"{args.api_url}/api/v1/workspaces", token) or []
    for ws in workspaces:
        if ws.get("name") == args.workspace_name:
            print(f"Reusing workspace '{args.workspace_name}' ({ws['id']})")
            return ws["id"]
    created = http_json("POST", f"{args.api_url}/api/v1/workspaces", token, {
        "name": args.workspace_name,
        "description": "Seeded by seed-dashboard-data.py",
    })
    print(f"Created workspace '{args.workspace_name}' ({created['id']})")
    return created["id"]


def ensure_domain(args, token):
    shared = http_json("GET", f"{args.api_url}/api/v1/domains/shared", token) or []
    if shared:
        return shared[0]["id"], shared[0]["name"]
    raise RuntimeError(
        "No shared domain found (expected one from shared_domains in management-api config). "
        "Pass --domain-id to use a specific domain instead."
    )


ROUTE_SPECS = [
    # link, destination, base daily clicks (before weekday/trend jitter)
    ("promo-fall", "https://shop.example.com/fall-sale", 45),
    ("blog-launch", "https://blog.example.com/new-product-launch", 30),
    ("docs-api", "https://docs.example.com/api/v2", 20),
    ("webinar-oct", "https://events.example.com/webinar-october", 15),
    ("newsletter", "https://example.com/newsletter/signup", 10),
    ("support", "https://support.example.com/contact", 8),
]


def ensure_routes(args, token, workspace_id, domain_id):
    existing = http_json(
        "GET",
        f"{args.api_url}/api/v1/routes?workspaceId={workspace_id}&pageSize=100",
        token,
    )
    by_link = {r["link"]: r for r in (existing or {}).get("data", [])}

    routes = []
    for link, dest, base_clicks in ROUTE_SPECS:
        existing_route = by_link.get(link)
        if existing_route:
            print(f"Reusing route '{link}' ({existing_route['id']})")
            routes.append((existing_route["id"], dest, base_clicks))
            continue
        created = http_json("POST", f"{args.api_url}/api/v1/routes", token, {
            "link": link,
            "dest": dest,
            "domainId": domain_id,
            "properties": {"workspaceId": workspace_id},
        })
        print(f"Created route '{link}' ({created['id']})")
        routes.append((created["id"], dest, base_clicks))
    return routes


COUNTRIES = [
    ("NA", "US", "New York"), ("NA", "US", "San Francisco"), ("EU", "GB", "London"),
    ("EU", "DE", "Berlin"), ("EU", "FR", "Paris"), ("AS", "JP", "Tokyo"),
    ("AS", "IN", "Mumbai"), ("SA", "BR", "Sao Paulo"), ("OC", "AU", "Sydney"),
    ("NA", "CA", "Toronto"),
]

DEVICES = [
    ("Apple", "iPhone", "iOS", "17.1", "Mobile Safari", "17.1"),
    ("Samsung", "Galaxy S23", "Android", "14", "Chrome Mobile", "118.0"),
    ("Apple", "Mac", "Mac OS X", "14.1", "Safari", "17.1"),
    ("Generic", "PC", "Windows", "10", "Chrome", "118.0"),
    ("Generic", "PC", "Windows", "11", "Edge", "118.0"),
    ("Google", "Pixel 8", "Android", "14", "Chrome Mobile", "118.0"),
]

BOT_AGENTS = [
    ("Generic", "Spider", "Linux", "0", "Googlebot", "2.1"),
    ("Generic", "Spider", "Linux", "0", "bingbot", "2.0"),
]


def generate_rows(owner_id, workspace_id, routes, days):
    rows = []
    today = datetime.datetime.now(datetime.timezone.utc).date()
    for route_id, dest, base_clicks in routes:
        for day_offset in range(days, -1, -1):
            day = today - datetime.timedelta(days=day_offset)
            trend = 1.0 + (days - day_offset) * 0.015
            weekday_factor = 0.6 if day.weekday() >= 5 else 1.0
            clicks_today = max(1, int(base_clicks * trend * weekday_factor * random.uniform(0.7, 1.3)))

            for _ in range(clicks_today):
                created = datetime.datetime(
                    day.year, day.month, day.day,
                    random.randint(0, 23), random.randint(0, 59), random.randint(0, 59),
                    random.randint(0, 999) * 1000,
                )
                is_bot = 1 if random.random() < 0.08 else 0
                continent, country, location = random.choice(COUNTRIES)
                if is_bot:
                    device_brand, device_family, os_family, os_version, ua_family, ua_version = random.choice(BOT_AGENTS)
                else:
                    device_brand, device_family, os_family, os_version, ua_family, ua_version = random.choice(DEVICES)

                is_unique = 1 if random.random() < 0.65 else 0
                ip = f"{random.randint(1,223)}.{random.randint(0,255)}.{random.randint(0,255)}.{random.randint(1,254)}"
                session_clicks = random.randint(1, 4) if random.random() < 0.4 else 0
                session_first = (
                    created.strftime("%Y-%m-%d %H:%M:%S.%f")[:-3]
                    if session_clicks > 0 else "1970-01-01 00:00:00.000"
                )

                rows.append({
                    "id": f"{route_id}-{day_offset}-{len(rows)}",
                    "owner_id": owner_id,
                    "creator_id": owner_id,
                    "route_id": route_id,
                    "workspace_id": workspace_id,
                    "created": created.strftime("%Y-%m-%d %H:%M:%S.%f")[:-3],
                    "dest": dest,
                    "ip": ip,
                    "continent": continent,
                    "country": country,
                    "location": location,
                    "os_family": os_family,
                    "os_version": os_version,
                    "user_agent_family": ua_family,
                    "user_agent_version": ua_version,
                    "device_brand": device_brand,
                    "device_family": device_family,
                    "device_model": device_family,
                    "session_first": session_first,
                    "session_clicks": session_clicks,
                    "is_unique": is_unique,
                    "is_bot": is_bot,
                })
    return rows


def reseed_clickhouse(args, owner_id, rows):
    print(f"Deleting previously seeded clickstream rows for owner {owner_id} ...")
    clickhouse(args, f"ALTER TABLE click_stream DELETE WHERE owner_id = '{owner_id}' SETTINGS mutations_sync = 1")

    payload = "\n".join(json.dumps(r) for r in rows).encode()
    clickhouse(args, "INSERT INTO click_stream FORMAT JSONEachRow", data=payload)
    print(f"Inserted {len(rows)} clickstream rows")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--keycloak-url", default="http://localhost:8080")
    p.add_argument("--realm", default="shortas-dev")
    p.add_argument("--client-id", default="shortas-api")
    p.add_argument("--client-secret", default="shortas-api-secret")
    p.add_argument("--username", default="testuser")
    p.add_argument("--password", default="testpassword")
    p.add_argument("--api-url", default="http://localhost:8090")
    p.add_argument("--ch-url", default="http://localhost:8123")
    p.add_argument("--ch-user", default="default")
    p.add_argument("--ch-password", default="clickhouse")
    p.add_argument("--ch-database", default="shortas")
    p.add_argument("--workspace-name", default="Marketing Team")
    p.add_argument("--domain-id", default=None, help="Use this domain instead of the shared domain")
    p.add_argument("--days", type=int, default=21, help="How many days of clickstream history to generate")
    p.add_argument("--skip-clickhouse", action="store_true", help="Only seed Postgres (routes/workspace), skip clickstream data")
    args = p.parse_args()

    random.seed()

    print(f"Logging in as {args.username} ...")
    token = get_token(args)
    owner_id = decode_jwt_sub(token)
    print(f"Owner id: {owner_id}")

    workspace_id = ensure_workspace(args, token)

    if args.domain_id:
        domain_id, domain_name = args.domain_id, args.domain_id
    else:
        domain_id, domain_name = ensure_domain(args, token)
    print(f"Using domain '{domain_name}' ({domain_id})")

    routes = ensure_routes(args, token, workspace_id, domain_id)

    if not args.skip_clickhouse:
        rows = generate_rows(owner_id, workspace_id, routes, args.days)
        reseed_clickhouse(args, owner_id, rows)

    print()
    print("Done. Log in to the dashboard as:")
    print(f"  {args.username} / {args.password}")


if __name__ == "__main__":
    try:
        main()
    except RuntimeError as e:
        print(f"error: {e}", file=sys.stderr)
        sys.exit(1)
