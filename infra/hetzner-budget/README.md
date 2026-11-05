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
3. Note the server's public IPv4 once it's created.

### 3. DNS

Add **A records**, replacing `yourdomain.com` with your real domain, all
pointing at the server's IP:

| Host | Points to |
|---|---|
| `dashboard.yourdomain.com` | server IP |
| `api.yourdomain.com` | server IP |
| `auth.yourdomain.com` | server IP |
| `images.yourdomain.com` | server IP |
| `yourdomain.com` (apex) | server IP |

Propagation can take minutes to hours — fine to continue while it settles.

### 4. SSH in

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

### 5. Prep the box

```bash
curl -fsSL https://get.docker.com | sh
```

Then, from the Hetzner **Console** (not just `ufw` on the box, so a mistake
doesn't lock you out): **Firewalls → Create Firewall** — allow inbound TCP
22 (SSH), 80, 443, 4433, 5800 from `0.0.0.0/0`; deny everything else; attach
it to the server. All the data ports (27017, 5432, 6379, 8123, 5672,
9200, 9000...) have no business being internet-facing — Docker publishing
them on this box is for simplicity, not public access, so the firewall is
what actually keeps them private.

### 6. Get the code and make images pullable

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas/infra/hetzner-budget
```

Images are built by CI into GHCR (`ghcr.io/flyingcow/shortas/*`), but
**GitHub packages default to private** — `docker compose pull` will fail
with an auth error until you either make the 14 packages public (GitHub →
your account's **Packages** tab → each package's settings → Change
visibility → Public) or `docker login ghcr.io` on the server with a PAT
that has `read:packages` scope. Public is simpler for a single budget box
with nothing sensitive in the images.

### 7. Configure secrets

```bash
cp .env.example .env
nano .env
```

Fill in every `changeme-*` with real random values (`openssl rand -hex 24`
run on your own machine, paste the output in) and the hostnames from step
3. Leave `IMAGE_TAG=latest` for now.

### 8. Launch

```bash
docker compose pull
docker compose up -d
docker compose ps
```

Give it a couple of minutes — Mongo/ClickHouse/Elasticsearch/Fluvio all
need to come up before the app services connect successfully;
`docker compose ps` will show some containers restarting until their
dependencies are ready.

### 9. First login

- `https://auth.yourdomain.com` — Keycloak admin console, log in with
  `KEYCLOAK_ADMIN_USER`/`PASSWORD` from `.env`.
- `https://dashboard.yourdomain.com` — the app itself.

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
