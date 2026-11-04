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
  docker compose -f "$COMPOSE_FILE" --env-file .env pull
  docker compose -f "$COMPOSE_FILE" --env-file .env $SINGLETON_FLAG up -d
else
  docker compose -f "$COMPOSE_FILE" --env-file .env pull --ignore-pull-failures
  docker compose -f "$COMPOSE_FILE" --env-file .env up -d
fi

docker compose -f "$COMPOSE_FILE" --env-file .env ps
