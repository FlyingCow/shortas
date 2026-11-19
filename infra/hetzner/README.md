# Hetzner deployment

A four-node layout with a redundant app tier, about €220–250 a month. It is the same Docker Compose stack as local, split across hosts on a private network. There is no Kubernetes and no Terraform. For a single-VM version, see [`../hetzner-budget`](../hetzner-budget).

```
 Internet
   |-- LB "click" (TCP 443->4433, 80->5800) --+
   |                                          v
   |-- LB "app"   (TCP 443, 80) ------> app-node-1, app-node-2
   |                                    click-router(-api), api,
   |                                    click-aggregator-api, notification-worker,
   |                                    verifiers, route-icon-worker,
   |                                    dashboard, landing, pages, Caddy
   |                                    + node 1 only: click-tracker,
   |                                      click-aggregator, cert-bot
   |
   |-- DNS --> platform-node: Keycloak, Prometheus, Grafana, Loki, Caddy
   |
   +   data-node (+ Volume): Mongo, Postgres x2, ClickHouse, Redis,
                             Elasticsearch, RabbitMQ, Fluvio, safe-browsing

 Hetzner Object Storage: route images, notification templates, Loki chunks
```

**Singletons.** Click Tracker and Click Aggregator use named Fluvio consumers on single-partition topics, and two copies would double-process or lose offsets. Two copies of Cert Bot would race on issuance and burn through ACME rate limits. Everything else is stateless or a RabbitMQ competing consumer, so it is safe to run on both app nodes.

## One-time setup in the Hetzner console

1. **Private network**, e.g. `10.0.0.0/16`. Attach all four servers and note their private IPs.
2. **Servers**: data-node CCX23, platform-node CX43, app-node-1 and app-node-2 CPX32. Install Docker on each:

   ```bash
   curl -fsSL https://get.docker.com | sh
   ```

3. **Volume** (e.g. 300 GB) on the data node, mounted at e.g. `/mnt/shortas-data`. The compose file uses named Docker volumes. Either move Docker's data-root onto the volume, or bind-mount the stateful volumes in `docker-compose.data.yml` under it.
4. **Object Storage**: create a bucket and credentials in the same region.
5. **Load balancers** (two LB11, TCP passthrough, targets are both app nodes on their private IPs):
   - **click**: 443→4433 and 80→5800, health check on 5800.
   - **app**: 443→443 and 80→80, health check on 80. Caddy terminates TLS behind it.
6. **Firewalls**: data and platform nodes allow only SSH from your admin IP. App nodes allow SSH from your admin IP, plus 80 and 443 from the load balancers.
7. **DNS**:
   - short-link domains → LB click
   - apex, `dashboard.` and `api.` → LB app
   - `auth.` and `grafana.` → the platform node's public IP

   Put LB click's IP (comma-separated if there are several) in `DOMAIN_ALLOWED_IPV4` in `.env` on **both** app nodes. Domain Verifier accepts custom domains that point there, and `deploy.sh app` refuses to run without it.

## Deploy

Every node needs a full checkout, because the compose files reference sibling paths. Every node also needs access to the GHCR images: make the packages public, or run `docker login ghcr.io` with a token that has `read:packages`.

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas/infra/hetzner
cp .env.<role>.example .env              # data | platform | app
./deploy.sh <data|platform|app>          # app-node-2 too
./deploy.sh app --singleton              # app-node-1 only
```

Deploy in this order: the data node, then the platform node, then both app nodes. To upgrade, run `git pull`, bump `IMAGE_TAG` if you pin it, and re-run `./deploy.sh app` on one app node at a time. The load balancer drains the node being updated.

## Before going live

- Replace every `changeme-*` with `openssl rand -hex 24`. A shared secret must have the same value in every node's `.env`.
- Put your own Google Safe Browsing key in `.env.data`'s `GSB_API_KEY`.
- Cert Bot uses the Let's Encrypt **staging** directory. Once issuance works, set `CERTBOT__ACME__DIRECTORY_URL` on the `cert-bot` container (or change `workers/cert-bot/config/production.toml` and rebuild).
- Configure real SMTP or SES credentials (`EMAIL_*`, `SMTP_*` in `.env.app`).

`keycloak-realm-setup` runs on the platform node at every start. It points the dashboard's redirect URIs at `DASHBOARD_HOSTNAME` and deletes the dev `testuser`. Check its logs if login fails.

## Backups

- Turn on Hetzner automated backups for the data and platform nodes. App nodes are stateless.
- Schedule snapshots of the data node's volume.
- Databases aren't replicated. Test a restore before you need one.
