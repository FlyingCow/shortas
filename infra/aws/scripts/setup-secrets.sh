#!/bin/bash
set -euo pipefail

# Setup AWS Secrets Manager Secrets
# Usage: ./setup-secrets.sh <environment>

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

usage() {
    echo "Usage: $0 <environment>"
    echo ""
    echo "Arguments:"
    echo "  environment    Environment (dev|prod)"
    echo ""
    echo "This script creates or updates secrets in AWS Secrets Manager."
    echo "Secrets are generated automatically - existing values are preserved."
    exit 1
}

generate_password() {
    local length="${1:-32}"
    openssl rand -base64 48 | tr -dc 'a-zA-Z0-9' | head -c "${length}"
}

create_or_update_secret() {
    local secret_name="$1"
    local secret_value="$2"
    local description="${3:-}"

    log_step "Processing secret: ${secret_name}"

    # Check if secret exists
    if aws secretsmanager describe-secret --secret-id "${secret_name}" > /dev/null 2>&1; then
        log_info "Secret exists, updating..."
        aws secretsmanager update-secret \
            --secret-id "${secret_name}" \
            --secret-string "${secret_value}" \
            --description "${description}" > /dev/null
    else
        log_info "Creating new secret..."
        aws secretsmanager create-secret \
            --name "${secret_name}" \
            --secret-string "${secret_value}" \
            --description "${description}" > /dev/null
    fi

    log_info "Secret ${secret_name} configured"
}

get_existing_secret() {
    local secret_name="$1"
    local key="$2"
    local default="$3"

    local value
    value=$(aws secretsmanager get-secret-value --secret-id "${secret_name}" --query 'SecretString' --output text 2>/dev/null | jq -r ".${key} // empty" 2>/dev/null || echo "")

    if [[ -n "${value}" ]]; then
        echo "${value}"
    else
        echo "${default}"
    fi
}

# Main execution
ENVIRONMENT="${1:-}"

if [[ -z "${ENVIRONMENT}" ]]; then
    log_error "Environment is required"
    usage
fi

if [[ "${ENVIRONMENT}" != "dev" && "${ENVIRONMENT}" != "prod" ]]; then
    log_error "Invalid environment: ${ENVIRONMENT}"
    usage
fi

log_info "Setting up secrets for ${ENVIRONMENT} environment"
log_info "Region: $(aws configure get region || echo 'us-east-1')"

# RDS Credentials
log_step "Configuring RDS credentials..."
RDS_USERNAME="shortas_admin"
RDS_PASSWORD=$(get_existing_secret "shortas/${ENVIRONMENT}/rds" "password" "$(generate_password 32)")
RDS_SECRET=$(cat <<EOF
{
    "username": "${RDS_USERNAME}",
    "password": "${RDS_PASSWORD}",
    "engine": "postgres",
    "dbname": "shortas"
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/rds" "${RDS_SECRET}" "RDS Aurora PostgreSQL credentials for Shortas ${ENVIRONMENT}"

# Redis Credentials
log_step "Configuring Redis credentials..."
REDIS_AUTH=$(get_existing_secret "shortas/${ENVIRONMENT}/redis" "auth_token" "$(generate_password 32)")
REDIS_SECRET=$(cat <<EOF
{
    "auth_token": "${REDIS_AUTH}",
    "port": 6379
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/redis" "${REDIS_SECRET}" "ElastiCache Redis credentials for Shortas ${ENVIRONMENT}"

# RabbitMQ Credentials
log_step "Configuring RabbitMQ credentials..."
MQ_USERNAME="shortas_admin"
MQ_PASSWORD=$(get_existing_secret "shortas/${ENVIRONMENT}/rabbitmq" "password" "$(generate_password 32)")
MQ_SECRET=$(cat <<EOF
{
    "username": "${MQ_USERNAME}",
    "password": "${MQ_PASSWORD}"
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/rabbitmq" "${MQ_SECRET}" "Amazon MQ RabbitMQ credentials for Shortas ${ENVIRONMENT}"

# ClickHouse Credentials
log_step "Configuring ClickHouse credentials..."
CH_USERNAME="default"
CH_PASSWORD=$(get_existing_secret "shortas/${ENVIRONMENT}/clickhouse" "password" "$(generate_password 32)")
CH_SECRET=$(cat <<EOF
{
    "username": "${CH_USERNAME}",
    "password": "${CH_PASSWORD}",
    "database": "shortas"
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/clickhouse" "${CH_SECRET}" "ClickHouse credentials for Shortas ${ENVIRONMENT}"

# Keycloak Admin Credentials
log_step "Configuring Keycloak credentials..."
KC_USERNAME="admin"
KC_PASSWORD=$(get_existing_secret "shortas/${ENVIRONMENT}/keycloak" "admin_password" "$(generate_password 32)")
KC_SECRET=$(cat <<EOF
{
    "admin_username": "${KC_USERNAME}",
    "admin_password": "${KC_PASSWORD}"
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/keycloak" "${KC_SECRET}" "Keycloak admin credentials for Shortas ${ENVIRONMENT}"

# API Secrets (JWT, encryption keys)
log_step "Configuring API secrets..."
JWT_SECRET=$(get_existing_secret "shortas/${ENVIRONMENT}/api" "jwt_secret" "$(generate_password 64)")
ENCRYPTION_KEY=$(get_existing_secret "shortas/${ENVIRONMENT}/api" "encryption_key" "$(generate_password 32)")
API_SECRET=$(cat <<EOF
{
    "jwt_secret": "${JWT_SECRET}",
    "encryption_key": "${ENCRYPTION_KEY}"
}
EOF
)
create_or_update_secret "shortas/${ENVIRONMENT}/api" "${API_SECRET}" "API secrets for Shortas ${ENVIRONMENT}"

echo ""
log_info "========== Secrets Setup Complete =========="
log_info "Environment: ${ENVIRONMENT}"
log_info ""
log_info "Created/updated secrets:"
echo "  - shortas/${ENVIRONMENT}/rds"
echo "  - shortas/${ENVIRONMENT}/redis"
echo "  - shortas/${ENVIRONMENT}/rabbitmq"
echo "  - shortas/${ENVIRONMENT}/clickhouse"
echo "  - shortas/${ENVIRONMENT}/keycloak"
echo "  - shortas/${ENVIRONMENT}/api"
echo ""
log_warn "Note: These secrets contain auto-generated passwords."
log_warn "Make sure to update Terraform with any manual secret references."
