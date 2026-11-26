# Hetzner budget deployment

The whole stack on one Hetzner VM for about €20 a month, suitable for a demo or low traffic. There is no redundancy: if the VM goes down, everything is down. For the multi-node layout, see [`../hetzner`](../hetzner). It uses the same images and the same env var names, so moving up means adding nodes.

| Item | Spec | Price / month (excl. VAT) |
|---|---|---|
| Server | CX43: 8 vCPU, 16 GB, 160 GB NVMe | ~€16 |
| Automated backups | 20% of server price | ~€3 |

16 GB is the practical minimum. The databases plus Keycloak and Fluvio idle at 6–10 GB. A CX33 (8 GB) can work if you leave monitoring off.

Compared with the local stack, this tier:

- pulls prebuilt images from GHCR instead of building them on the box;
- runs one Postgres for both the API and Keycloak (`postgres-init/`);
- keeps object storage on the box (RustFS behind `minio-nginx`);
- drops Vault, Mongo Express and MailHog;
- puts Prometheus, Grafana and Loki behind the `observability` profile, off by default.

## 1. Account and SSH key

Sign up at console.hetzner.com, then add your public key under **Security → SSH Keys**:

```bash
ls ~/.ssh/id_ed25519.pub 2>/dev/null || ssh-keygen -t ed25519 -C "your-email"
```

## 2. Server

**Servers → Add Server**: Ubuntu 24.04, **Shared vCPU → CX43**, and select your SSH key. Note the server's public IPv4. This is the *primary IP*.

## 3. Second IP for short links

Caddy serves the app's hostnames on ports 443 and 80. Click Router serves the short-link domains with its own TLS, so it needs a second IP.

1. **Floating IPs → Create Floating IP**: IPv4, same location, assigned to this server. This is `SHORTLINK_IP`.
2. After you SSH in (step 5), configure it on the interface:

   ```bash
   ip -4 addr show                       # find the interface name
   nano /etc/netplan/60-floating-ip.yaml
   ```

   ```yaml
   network:
     version: 2
     renderer: networkd
     ethernets:
       eth0:                 # the interface name from above
         addresses:
           - YOUR.FLOATING.IP.HERE/32
   ```

   ```bash
   sudo netplan apply        # briefly drops networking
   ip -4 addr show           # both IPs should be listed
   ```

## 4. DNS

| Host | A record |
|---|---|
| `yourdomain.com`, `dashboard.`, `api.`, `auth.`, `images.` | primary IP |
| short-link domains, e.g. `shortas.space` | Floating IP |

Set `DOMAIN_ALLOWED_IPV4` in `.env` to the Floating IP. Domain Verifier accepts a custom domain when one of its A records is in this comma-separated list, and the dashboard shows these IPs in its DNS instructions. `DOMAIN_ALLOWED_IPV6` works the same way and can stay empty.

## 5. SSH in

```bash
ssh root@<server-ip>
```

If it asks for a password, no key was attached. Use the root password Hetzner emailed you, then add your key:

```bash
mkdir -p ~/.ssh && chmod 700 ~/.ssh
echo "PASTE_THE_KEY_LINE_HERE" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
```

If you see `REMOTE HOST IDENTIFICATION HAS CHANGED`, it's because Hetzner reuses IPs:

```bash
ssh-keygen -R <server-ip>
ssh root@<server-ip>
```

## 6. Docker and firewall

```bash
curl -fsSL https://get.docker.com | sh
```

In the Hetzner console, go to **Firewalls → Create Firewall**. Allow inbound TCP 22, 80 and 443, deny everything else, and attach the firewall to the server. It covers both IPs and keeps the database ports private.

## 7. Code and images

```bash
git clone https://github.com/FlyingCow/shortas.git
cd shortas/infra/hetzner-budget
```

GHCR packages (`ghcr.io/flyingcow/shortas/*`) are private by default. Either make them public in GitHub's package settings, or run `docker login ghcr.io` with a token that has `read:packages`.

## 8. Secrets

```bash
cp .env.example .env
nano .env
```

Replace every `changeme-*` with output from `openssl rand -hex 24`. Then fill in the hostnames, `PRIMARY_IP`, `SHORTLINK_IP`, `DOMAIN_ALLOWED_IPV4`, SMTP settings, `ACME_ACCOUNT_EMAIL` and your own `GSB_API_KEY`. Keep `IMAGE_TAG=latest`.

## 9. Launch

```bash
docker compose pull
docker compose up -d
docker compose ps
```

Some containers restart for a few minutes until Mongo, ClickHouse, Elasticsearch and Fluvio are ready. Monitoring is optional:

```bash
docker compose --profile observability up -d
```

## 10. First login

On every start, the one-shot `keycloak-realm-setup` container points the dashboard's redirect URIs at `DASHBOARD_HOSTNAME`, configures SMTP, and deletes the dev `testuser`. Check `docker compose logs keycloak-realm-setup` if login fails.

- `https://auth.yourdomain.com`: Keycloak admin, using `KEYCLOAK_ADMIN_USER` / `KEYCLOAK_ADMIN_PASSWORD`
- `https://dashboard.yourdomain.com`: register an account (registration is open)

## Before going live

- Cert Bot uses the Let's Encrypt **staging** directory, whose certificates browsers reject. Once issuance works, set `ACME_DIRECTORY_URL=https://acme-v02.api.letsencrypt.org/directory` in `.env` and run `docker compose up -d cert-bot`. Cert Bot creates a production account by itself.
- Use your own Google Safe Browsing key and real SMTP credentials.

## Test data

### Demo analytics

The seed script needs a registered account, because `testuser` is deleted. Keycloak and ClickHouse aren't published to the host, so run it inside the Docker network:

```bash
cd ~/shortas/infra/hetzner-budget
export CH_PASS=$(grep -E '^CLICKHOUSE_PASSWORD=' .env | cut -d= -f2-)

docker run --rm --network shortas-net \
  -v "$(pwd)/../custom/seed-dashboard-data.py:/seed.py:ro" \
  python:3.12-alpine python3 /seed.py \
  --keycloak-url http://keycloak:8080 \
  --api-url http://api:8080 \
  --ch-url http://clickhouse:8123 \
  --ch-password "$CH_PASS" \
  --username your@email.com \
  --password yourpassword \
  --days 30
```

### Safe Browsing test URLs

To check that blocking works, point a route at one of Google's permanent test URLs and wait for the next Route Verifier pass (5 minutes by default). The route should switch to `Blocked`.

- `http://testsafebrowsing.appspot.com/s/malware.html`
- `http://testsafebrowsing.appspot.com/s/phishing.html`
- `http://testsafebrowsing.appspot.com/s/unwanted.html`

If nothing happens, check the `safe-browsing` logs. It has to finish syncing Google's threat lists first.
