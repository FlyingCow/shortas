# Hetzner deployment

A 4-node production-ready layout, built directly from the existing
`infra/custom/docker-compose.yml` stack rather than a parallel Terraform
setup (Hetzner has no managed Postgres/Redis/Elasticsearch/MQ equivalents —
you're self-hosting these either way, so plain Docker Compose per node is
the simplest honest option). For a cheaper single-VM variant, see "Budget
single-node" at the bottom.

## Topology

```
                    Hetzner Cloud Network (private, 10.0.0.0/16)
                    ┌──────────────────────────────────────────┐
  Internet          │                                          │
     │              │  ┌─────────────┐      ┌─────────────┐    │
     ├─ LB "click" ─┼──┤ app-node-1  │      │ app-node-2  │    │
     │  (TCP passthrough to          │      │             │    │
     │   click-router 4433/5800)     │      │             │    │
     │              │  │ click-router(-api)  click-router(-api)│
     │              │  │ click-aggregator-api  (same, x2)  │    │
     │              │  │ management-api        (same, x2)  │    │
     │              │  │ notification-worker   (same, x2)  │    │
     ├─ LB "app" ───┼──┤ domain-verifier/route-verifier/    │    │
     │  (TCP passthrough │ route-icon-worker    (same, x2)  │    │
     │   to Caddy 443/80) │ pages, dashboard, landing (x2)  │    │
     │              │  │ Caddy (TLS: dashboard/landing/api) │    │
     │              │  │                                    │    │
     │              │  │ -- singleton profile, node 1 only --   │
     │              │  │ click-tracker, click-aggregator,  │    │
     │              │  │ cert-bot                            │    │
     │              │  └─────────────┘      └─────────────┘    │
     │              │                                          │
     ├─ DNS A record directly at platform node ──┐             │
     │              │  ┌─────────────────────────▼─┐           │
     │              │  │ platform-node               │          │
     │              │  │ keycloak, prometheus,        │          │
     │              │  │ grafana, loki, Caddy         │          │
     │              │  │ (TLS: auth/grafana)          │          │
     │              │  └─────────────┬────────────────┘          │
     │              │                │                           │
     │              │  ┌─────────────▼────────────────┐          │
     │              │  │ data-node (+ attached Volume)  │         │
     │              │  │ mongo, keycloak-db,            │         │
     │              │  │ shortas-api-postgres,          │         │
     │              │  │ clickhouse, redis,             │         │
     │              │  │ elasticsearch, rabbitmq,       │         │
     │              │  │ fluvio (sc+spu), safe-browsing │         │
     │              │  └────────────────────────────────┘         │
     │              └──────────────────────────────────────────┘
     └─ Object Storage (Hetzner, S3-compatible): route images,
        notification templates, Loki chunks
```

Nothing here needs Kubernetes or Terraform — it's the same Docker Compose
model the repo already uses locally, split across hosts. See the pricing
breakdown already discussed in chat (~€220-250/mo for this layout).

## Why some workers are singletons

- `click-tracker` and `click-aggregator` are Fluvio consumers using a named,
  offset-managed consumer (`redirect/click-tracker/src/adapters/fluvio`,
  `redirect/click-aggregator/src/adapters/fluvio`). The topics are created
  with a single partition. Running two copies against the same named
  consumer on a single-partition topic risks double-processing or dropped
  offsets — it is **not** the same safe pattern as RabbitMQ competing
  consumers. Keep these on one node until/unless you deliberately
  repartition the topics and adopt per-instance consumer names.
- `cert-bot` polls Mongo on a timer and talks to Let's Encrypt's ACME API.
  Running two copies risks duplicate issuance races and burning your ACME
  rate limit. Keep it singleton.
- Everything else in `docker-compose.app.yml` (click-router, click-router-api,
  click-aggregator-api, management-api, notification-worker,
  domain-verifier, route-verifier, route-icon-worker, dashboard, landing,
  pages) is either stateless or a RabbitMQ competing consumer — safe, and
  intended, to run on both app nodes.

## One-time Hetzner console setup

1. **Private Network**: create one (e.g. `10.0.0.0/16`), attach all 4
   servers, note the private IP Hetzner assigns each.
2. **Servers**: data-node (CCX23), platform-node (CX43), app-node-1 +
   app-node-2 (CPX32 each) — see pricing discussion above for sizing
   trade-offs. Install Docker + the Compose plugin on each
   (`curl -fsSL https://get.docker.com | sh`).
3. **Volume**: attach a Hetzner Volume (e.g. 300GB) to the data node, mount
   it (e.g. at `/mnt/shortas-data`), and point the stateful `volumes:` in
   `docker-compose.data.yml` at paths under it if you want volume-level
   snapshots independent of the boot disk — the compose file as written
   uses named Docker volumes, which already live on whatever disk Docker's
   data-root is on; repoint `/var/lib/docker` or edit the compose volumes to
   bind-mount under `/mnt/shortas-data` if you want that separation.
4. **Object Storage**: create a bucket + API credentials in the Hetzner
   Console (Object Storage is region-scoped, e.g. `nbg1`). Used for
   route-images/notification-templates (replacing MinIO) and optionally
   Loki chunks.
5. **Load Balancers**: two LB11s.
   - **LB "click"**: TCP passthrough, listen 443→4433 and 80→5800, targets
     = app-node-1 + app-node-2 (private IPs), health check against
     click-router's health endpoint on 5800.
   - **LB "app"**: TCP passthrough, listen 443→443 and 80→80, targets =
     app-node-1 + app-node-2, health check on 80. This hits Caddy, which
     does the actual TLS termination/host routing for dashboard/landing/api
     — passthrough here, not LB-terminated TLS, so Caddy's Let's Encrypt
     HTTP-01 challenge keeps working unmodified.
6. **Cloud Firewall**: data-node and platform-node get NO public inbound
   rules except SSH from your admin IP (lock it to that /32). App nodes
   allow SSH from your admin IP plus 443/80 from the two LBs' source
   ranges (Hetzner publishes these). Nothing other than the LBs should ever
   reach the app nodes directly.
7. **DNS**: short-link domains / `shortas.space` equivalent → LB "click"'s
   IP. `dashboard.`/`api.`/apex → LB "app"'s IP. `auth.`/`grafana.` → the
   platform node's own public IP directly (single instance, no LB).

## Deploying

Each node needs a full checkout of this repo (compose files reference
sibling paths like `../../redirect/click-aggregator-api/migrations` and
`../../notification-templates`), plus images already pushed to GHCR by
`.github/workflows/docker-publish.yml` (make the GHCR packages public, or
`docker login ghcr.io` with a `read:packages` PAT on each node).

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas/infra/hetzner
cp .env.<role>.example .env   # data | platform | app — fill in real values
./deploy.sh <data|platform|app>          # app-node-2 too
./deploy.sh app --singleton              # app-node-1 ONLY
```

Order: data node first (everything else depends on it being up), then
platform node, then both app nodes (app-node-1 with `--singleton`,
app-node-2 without). To roll out a new version: `git pull`, bump
`IMAGE_TAG` in `.env` if you're pinning tags, re-run `./deploy.sh app` on
each app node in turn (not simultaneously, if you want a rolling deploy —
the LB health check will drain the one being updated).

## Things you must change before this is actually production

- **Secrets**: every `.env.*.example` has `changeme-*` placeholders.
  Generate real random values (`openssl rand -hex 24`) — the same value
  must appear in both `.env.data` and whichever `.env.app`/`.env.platform`
  reference it.
- **`redirect/route-verifier/.env` has a live-looking Google Safe Browsing
  API key committed to git in this repo.** Rotate it in the Google Cloud
  Console and put the new one only in `.env.data`'s `GSB_API_KEY` — don't
  reuse the committed one.
- **cert-bot defaults to the Let's Encrypt *staging* ACME directory**
  (`redirect/cert-bot/config/production.toml`). Verify issuance works
  end-to-end on staging first, then switch to the production directory
  (`CERTBOT__ACME__DIRECTORY_URL` or edit the config) — staging certs are
  untrusted by browsers.
- **Mailhog is dev-only.** Fill in real SMTP/SES credentials in
  `.env.app`'s `EMAIL_*`/`SMTP_*` vars before invites/notifications matter.
- **GHCR package visibility**: either make the 14 packages public or set up
  a pull credential on every node — `docker compose pull` will fail
  otherwise.

## Backups

- Hetzner automated backups (20% of server price) on the data node and
  platform node. App nodes are stateless — just redeploy from the image
  registry, no backup needed.
- Additionally snapshot the data node's Volume on a schedule (Hetzner
  Volumes support manual + scripted snapshots) for a faster/cheaper restore
  path than a full server backup.
- There is no multi-node database replication here (see cost discussion —
  that's a deliberate trade-off, not an oversight). Test your restore
  procedure before you need it.

## Budget single-node

For a demo/low-traffic deploy, skip all of the above — see
`../hetzner-budget/` for a dedicated single-VM layout (one CX43, prebuilt
GHCR images, self-hosted MinIO, ~€20/mo). It's the direct downgrade path
from this directory: same application images, same env var shapes, just
one node instead of four and no HA.
