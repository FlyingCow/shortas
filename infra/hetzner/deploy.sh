#!/bin/bash
# Run ON a Hetzner node (data|platform|app) from inside a checkout of this
# repo, after creating infra/hetzner/.env from the matching .env.<role>.example.
# Usage: infra/hetzner/deploy.sh <data|platform|app> [--singleton]
set -euo pipefail

ROLE="${1:-}"
SINGLETON_FLAG=""
[[ "${2:-}" == "--singleton" ]] && SINGLETON_FLAG="--profile singleton"

case "$ROLE" in
  data|platform|app) ;;
  *) echo "Usage: $0 <data|platform|app> [--singleton]"; exit 1 ;;
esac

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

COMPOSE_FILE="docker-compose.${ROLE}.yml"
[[ -f .env ]] || { echo ".env not found — copy .env.${ROLE}.example to .env and fill it in first"; exit 1; }

if [[ "$ROLE" == "app" ]]; then
  # domain-verifier accepts a custom domain only if one of its A records is in
  # this list, and the dashboard shows these IPs as the records to set.
  ALLOWED_IPV4="$(grep -E '^DOMAIN_ALLOWED_IPV4=' .env | cut -d= -f2- | tr -d '"' || true)"
  if [[ -z "$ALLOWED_IPV4" ]]; then
    echo "DOMAIN_ALLOWED_IPV4 is not set in .env — set it to LB \"click\"'s public IP(s), comma-separated"; exit 1
  fi
  if [[ "$ALLOWED_IPV4" == *203.0.113.* ]]; then
    echo "WARNING: DOMAIN_ALLOWED_IPV4 ($ALLOWED_IPV4) still contains the documentation placeholder range 203.0.113.0/24"
  fi
  docker compose -f "$COMPOSE_FILE" --env-file .env pull
  docker compose -f "$COMPOSE_FILE" --env-file .env $SINGLETON_FLAG up -d
else
  docker compose -f "$COMPOSE_FILE" --env-file .env pull --ignore-pull-failures
  docker compose -f "$COMPOSE_FILE" --env-file .env up -d
fi

docker compose -f "$COMPOSE_FILE" --env-file .env ps
