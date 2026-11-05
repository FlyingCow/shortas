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

## Setup

1. Create a Hetzner CX43 (or CX33 if you're comfortable with the RAM risk),
   install Docker + Compose plugin (`curl -fsSL https://get.docker.com | sh`).
2. Point DNS A records for `DASHBOARD_HOSTNAME`, `LANDING_HOSTNAME`,
   `API_HOSTNAME`, `S3_PUBLIC_HOSTNAME`, `KEYCLOAK_HOSTNAME` (and
   `GRAFANA_HOSTNAME` if you'll enable observability) at the box's public
   IP, plus whatever short-link domain(s) click-router will serve.
3. Hetzner Cloud Firewall: allow 22 (SSH, your IP only), 80/443 (public),
   and leave everything else closed — all the data ports (27017, 5432,
   6379, 8123, 5672, 9200, 9000...) have no business being internet-facing;
   Docker's published ports on this box are for container-to-container
   traffic model simplicity, not for public access, so firewall them even
   though compose publishes them.
4. `git clone` this repo onto the box (compose references sibling paths
   like `../../notification-templates` and `../custom/keycloak/...`).
5. `cd infra/hetzner-budget && cp .env.example .env` and fill in every
   `changeme-*` with real random values, plus your real hostnames/SMTP
   creds.
6. Make the GHCR packages public, or `docker login ghcr.io` on the box with
   a `read:packages` PAT.
7. `docker compose pull && docker compose up -d`

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
