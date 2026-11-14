# Hetzner budget deployment

A single-VM deployment for a demo, low-traffic launch, or anything where
€20/month beats HA. For the multi-node, no-single-point-of-failure-on-the-
hot-path layout, see `../hetzner/` instead — read its README's topology
section for the trade-offs this tier deliberately gives up (no replication
on any database, one copy of every app service, one VM is a full outage).

## Sizing / cost

| Item | Spec | Price/mo (excl. VAT) |
|---|---|---|
| Server | Hetzner CX43 — 8 vCPU / 16GB / 160GB NVMe | ~€16 |
| Automated backups | 20% of server price | ~€3 |
| **Total** | | **~€19-20** |

16GB is the realistic floor: Mongo + ClickHouse + Elasticsearch +
Keycloak(JVM) + Fluvio together idle around 6-10GB, with ClickHouse/ES
spiking under load. CX33 (8GB, ~€8.50/mo) is possible but tight — leave the
`observability` profile off and watch `docker stats` before trusting it
under real traffic.

## What's different from `../hetzner/` (production)

- **One Docker network, no cross-host wiring.** All 25+ containers run on
  one box, so container-name DNS just works — no `extra_hosts`, no private
  network, no per-node `.env` files to keep in sync.
- **Prebuilt GHCR images, not `build:`.** Compiling 9+ Rust crates with
  `cargo build --release` needs real RAM per crate; doing that on the same
  box serving traffic risks OOM-killing the database containers mid-build.
  Images come from `.github/workflows/docker-publish.yml`, same as the
  production tier.
- **Self-hosted MinIO/RustFS, not Hetzner Object Storage.** On a single box
  there's no durability win from paying ~€6.50/mo for Object Storage — this
  tier stays free beyond the one VM. `minio-nginx` (copied from
  `infra/custom/minio-nginx.conf`) handles the presigned-URL CORS proxying,
  same as local dev.
- **One Postgres instance, two databases.** `postgres-init/` creates both
  the API database and Keycloak's database in the same container to save
  RAM/overhead — see those two scripts for exactly what they do.
- **Vault, mongo-express, and Mailhog are dropped entirely** (unused by the
  app / dev-only) to keep headroom for the services that matter.
- **Monitoring is opt-in.** Prometheus/Grafana/Loki are behind the
  `observability` Compose profile, off by default. Enable with
  `docker compose --profile observability up -d` once you've confirmed the
  box has RAM to spare — Loki here uses local filesystem storage, not S3,
  so there's no Object Storage dependency either.
- **No HA, anywhere.** One Mongo, one Postgres, one ClickHouse, one Redis,
  one RabbitMQ, one Fluvio SC/SPU, one copy of every app service. This VM
  going down is a full outage. That's the actual budget trade-off, not a
  detail — if you need to not have this conversation at 3am, use
  `../hetzner/` instead.

## Setup from zero (no Hetzner account yet)

### 1. Account + SSH key

1. Sign up at **console.hetzner.com** (needs a payment method).
2. On your own machine, check for an existing SSH key or make one:
   ```bash
   ls ~/.ssh/id_ed25519.pub 2>/dev/null || ssh-keygen -t ed25519 -C "your-email"
   ```
3. Hetzner Console → **Security → SSH Keys → Add SSH Key** → paste the
   contents of `~/.ssh/id_ed25519.pub`.

### 2. Create the server

1. **Projects → New Project**.
2. **Servers → Add Server**: Location closest to your users; Image
   **Ubuntu 24.04**; Type **Shared vCPU → CX43** (8 vCPU/16GB — don't go
   smaller for a first attempt, see "Sizing / cost" above); SSH key: select
   the one you just added (this is the step that's easy to miss — if you
   skip it, Hetzner falls back to emailing you a one-time root password
   instead of key auth, which is a worse starting position, not a blocker).
3. Note the server's public IPv4 (its **primary IP**) once it's created.

### 3. Add a second IP

The app's own hostnames (dashboard/api/auth/images) are served by Caddy on
standard ports 443/80. Short-link domains (whatever you shorten URLs
under — e.g. `shortas.space`) are served the same way, but by
`click-router`, which does its own per-domain TLS. Two different programs
can't both bind port 443 on the same IP, so this box needs a **second
IPv4 address**. A server only ever has one *Primary IP* — the feature you
actually want is a **Floating IP**, a separate addressable resource you
create once and attach to a server:

1. Hetzner Console → **Floating IPs → Create Floating IP** — same
   location/datacenter as your server, type IPv4, attach it to this
   server. Note the address — it's `SHORTLINK_IP` in `.env` (step 8).
2. Unlike the Primary IP, a Floating IP is **not** auto-configured on the
   box — you have to tell the OS about it yourself (do this once you're
   SSH'd in, step 5):
   ```bash
   ip -4 addr show   # confirm your real interface name — don't assume eth0
   nano /etc/netplan/60-floating-ip.yaml
   ```
   ```yaml
   network:
     version: 2
     renderer: networkd
     ethernets:
       eth0:              # ← the interface name from `ip -4 addr show`
         addresses:
           - YOUR.FLOATING.IP.HERE/32
   ```
   ```bash
   sudo netplan apply   # briefly resets networking — don't do this over a
                        # connection you can't afford to drop for a few seconds
   ip -4 addr show      # confirm both IPs now show up
   ```

### 4. DNS

Add **A records**, replacing `yourdomain.com` with your real domain:

| Host | Points to |
|---|---|
| `dashboard.yourdomain.com` | primary IP |
| `api.yourdomain.com` | primary IP |
| `auth.yourdomain.com` | primary IP |
| `images.yourdomain.com` | primary IP |
| `yourdomain.com` (apex, landing) | primary IP |
| your short-link domain(s), e.g. `shortas.space` | **second** IP |

Propagation can take minutes to hours — fine to continue while it settles.

### 5. SSH in

```bash
ssh root@<server-ip>
```

Two things commonly trip people up here:

- **It asks for a password even though you added a key.** Check your
  email for a one-time root password Hetzner sends when no key was
  attached to the server (easy to miss selecting it in step 2). Log in
  with that password, then add your key so you don't need it again. On
  YOUR machine, get the key's contents: `cat ~/.ssh/id_ed25519.pub`. Then,
  on the server:
  ```bash
  mkdir -p ~/.ssh && chmod 700 ~/.ssh
  echo "PASTE_THE_KEY_LINE_HERE" >> ~/.ssh/authorized_keys
  chmod 600 ~/.ssh/authorized_keys
  ```
- **"WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!"** — routine on a
  freshly created cloud server; Hetzner reuses IPs, so your machine's
  memory of who last held that IP's SSH key is stale, not a sign of
  tampering. Fix and retry:
  ```bash
  ssh-keygen -R <server-ip>
  ssh root@<server-ip>
  ```

### 6. Prep the box

```bash
curl -fsSL https://get.docker.com | sh
```

Then, from the Hetzner **Console** (not just `ufw` on the box, so a mistake
doesn't lock you out): **Firewalls → Create Firewall** — allow inbound TCP
22 (SSH) and 80/443 from `0.0.0.0/0`; deny everything else; attach it to
the server. Firewall rules apply to the server regardless of which of its
two IPs traffic arrives on, so this one rule covers both. All the data
ports (27017, 5432, 6379, 8123, 5672, 9200, 9000...) have no business
being internet-facing — Docker publishing them on this box is for
simplicity, not public access, so the firewall is what actually keeps
them private.

### 7. Get the code and make images pullable

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas/infra/hetzner-budget
```

Images are built by CI into GHCR (`ghcr.io/flyingcow/shortas/*`), but
**GitHub packages default to private** — `docker compose pull` will fail
with an auth error until you either make the 15 packages public (GitHub →
your account's **Packages** tab → each package's settings → Change
visibility → Public) or `docker login ghcr.io` on the server with a PAT
that has `read:packages` scope. Public is simpler for a single budget box
with nothing sensitive in the images.

### 8. Configure secrets

```bash
cp .env.example .env
nano .env
```

Fill in every `changeme-*` with real random values (`openssl rand -hex 24`
run on your own machine, paste the output in), the hostnames from step 4,
and `PRIMARY_IP`/`SHORTLINK_IP` from steps 2-3. Leave `IMAGE_TAG=latest`
for now.

### 9. Launch

```bash
docker compose pull
docker compose up -d
docker compose ps
```

Give it a couple of minutes — Mongo/ClickHouse/Elasticsearch/Fluvio all
need to come up before the app services connect successfully;
`docker compose ps` will show some containers restarting until their
dependencies are ready.

### 10. First login

`keycloak-realm-setup` (a one-shot container that runs automatically on
every `docker compose up`) already took care of the realm for you:
pointed the dashboard's OIDC redirect URIs at `DASHBOARD_HOSTNAME`
(they default to `localhost:3000` in `realm-export.json`, which would
otherwise break login with "Invalid parameter: redirect_uri"), configured
real SMTP so password-reset emails send, and deleted the seeded
`testuser`/`testpassword` account (fine for local dev, not something to
leave on an internet-facing box). Check its logs
(`docker compose logs keycloak-realm-setup`) if any of that seems off.

- `https://auth.yourdomain.com` — Keycloak admin console, log in with
  `KEYCLOAK_ADMIN_USER`/`PASSWORD` from `.env`.
- `https://dashboard.yourdomain.com` — the app itself. Register a real
  account from here (`testuser` is gone) — registration is open by
  default (`registrationAllowed: true`).

## Before calling this live

Same checklist as `../hetzner/README.md`:
- Rotate the Google Safe Browsing API key — don't reuse the one committed
  in `redirect/route-verifier/.env`.
- cert-bot defaults to the Let's Encrypt **staging** ACME directory
  (`redirect/cert-bot/config/production.toml`) — verify issuance works,
  then switch to production.
- Real SMTP/SES credentials — Mailhog isn't in this tier at all.

## Growing out of this tier

If traffic outgrows one box, `../hetzner/` is the direct upgrade path: same
application images, same env var shapes (the production compose files are
literally this same service set, split across 4 nodes with HA on the app
tier). You don't re-architect anything, you just add nodes.

## Test data

### Demo analytics data

The dashboard's charts and stats stay empty until routes actually get
clicks. To populate them without waiting on real traffic,
`infra/custom/seed-dashboard-data.py` logs in as a real user, creates (or
reuses) a workspace and routes via the Management API, and backfills
clickstream history directly into ClickHouse. It's safe to re-run — routes
and the workspace are reused by name, so running it again just adds more
history on top.

It needs a real account to log in as: `keycloak-realm-setup` deletes the
seeded `testuser`/`testpassword` on every boot (see "First login" above), so
register a real account from the dashboard first and pass its credentials
below. Also note that neither Keycloak nor ClickHouse publishes a port to
the host in this tier, so run the script from a throwaway container on
`shortas-net` rather than from your own machine:

```bash
cd ~/shortas/infra/hetzner-budget
export CH_PASS=$(grep -E '^CLICKHOUSE_PASSWORD=' .env | cut -d= -f2-)

docker run --rm --network shortas-net \
  -v "$(pwd)/../custom/seed-dashboard-data.py:/seed.py:ro" \
  python:3.12-alpine python3 /seed.py \
  --keycloak-url http://keycloak:8080 \
  --api-url http://management-api:8080 \
  --ch-url http://clickhouse:8123 \
  --ch-password "$CH_PASS" \
  --username your@email.com \
  --password yourpassword \
  --days 30
```

Run `python3 seed-dashboard-data.py --help` for the full flag list (workspace
name, domain ID, day count, `--skip-clickhouse`, etc.) — the defaults for
realm/client ID/secret already match this deployment's `realm-export.json`,
so only the credentials and endpoints above need overriding.

### Safe Browsing test URLs

To verify the `route-verifier` → `safe-browsing` blocking flow without a real
malicious link, point a route's destination at one of Google's official Safe
Browsing test URLs — permanent entries in the real threat lists, published
specifically for this purpose:

- `http://testsafebrowsing.appspot.com/s/malware.html`
- `http://testsafebrowsing.appspot.com/s/phishing.html`
- `http://testsafebrowsing.appspot.com/s/unwanted.html`

`route-verifier` checks on its usual interval (`check_interval_seconds`,
default 5 minutes — see `redirect/route-verifier/README.md`); the route
should flip to `Blocked` with a reason like `Safe Browsing: MALWARE`. If it
never does, check the `safe-browsing` service's logs first — it needs its
own `GSB_API_KEY` to have finished syncing Google's threat list data before
it can flag anything.