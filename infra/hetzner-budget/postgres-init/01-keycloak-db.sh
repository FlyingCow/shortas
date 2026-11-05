#!/bin/bash
# Creates Keycloak's own database + user in the SAME Postgres instance as
# the API database, to save a whole container on a budget single-node box.
# Needs KEYCLOAK_DB_NAME/USER/PASSWORD passed into this container's
# environment (see docker-compose.yml).
set -euo pipefail

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER ${KEYCLOAK_DB_USER} WITH PASSWORD '${KEYCLOAK_DB_PASSWORD}';
    CREATE DATABASE ${KEYCLOAK_DB_NAME} OWNER ${KEYCLOAK_DB_USER};
EOSQL
