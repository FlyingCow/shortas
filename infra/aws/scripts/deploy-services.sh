#!/bin/bash
set -euo pipefail

# Deploy Services to ECS
# Usage: ./deploy-services.sh <environment> [image_tag] [service1 service2 ...]

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

# Service deployment order (dependencies first)
DEPLOYMENT_ORDER=(
    # Infrastructure services first
    "fluvio-sc"
    "fluvio-spu"
    # Core services
    "pages"
    "click-router"
    "click-router-api"
    # Analytics pipeline
    "click-tracker"
    "click-aggregator"
    "click-aggregator-api"
    # Support services
    "domain-verifier"
    "route-verifier"
    "route-icon-worker"
    "cert-bot"
    # API and frontend
    "shortas-api"
    "dashboard"
    "landing"
    # Auth
    "keycloak"
)

usage() {
    echo "Usage: $0 <environment> [image_tag] [service1 service2 ...]"
    echo ""
    echo "Arguments:"
    echo "  environment    Environment (dev|prod)"
    echo "  image_tag      Docker image tag (default: latest)"
    echo "  services       Optional list of services (default: all in dependency order)"
    echo ""
    echo "Examples:"
    echo "  $0 dev                          # Deploy all services with latest tag"
    echo "  $0 prod abc123                  # Deploy all services with specific tag"
    echo "  $0 dev latest click-router      # Deploy only click-router"
    exit 1
}

get_cluster_name() {
    echo "${ENVIRONMENT}-shortas-cluster"
}

update_service() {
    local service="$1"
    local cluster="$2"
    local ecs_service="${ENVIRONMENT}-${service}"

    log_step "Updating service: ${ecs_service}"

    # Check if service exists
    if ! aws ecs describe-services --cluster "${cluster}" --services "${ecs_service}" --query 'services[0].serviceName' --output text 2>/dev/null | grep -q "${ecs_service}"; then
        log_warn "Service ${ecs_service} not found, skipping..."
        return 0
    fi

    # Force new deployment
    aws ecs update-service \
        --cluster "${cluster}" \
        --service "${ecs_service}" \
        --force-new-deployment \
        --query 'service.serviceName' \
        --output text

    log_info "Triggered deployment for ${ecs_service}"
}

wait_for_service() {
    local service="$1"
    local cluster="$2"
    local ecs_service="${ENVIRONMENT}-${service}"
    local timeout=300
    local interval=10
    local elapsed=0

    log_step "Waiting for ${ecs_service} to stabilize..."

    while [[ $elapsed -lt $timeout ]]; do
        local status
        status=$(aws ecs describe-services \
            --cluster "${cluster}" \
            --services "${ecs_service}" \
            --query 'services[0].deployments[?status==`PRIMARY`].rolloutState' \
            --output text 2>/dev/null || echo "UNKNOWN")

        if [[ "${status}" == "COMPLETED" ]]; then
            log_info "${ecs_service} deployment completed"
            return 0
        elif [[ "${status}" == "FAILED" ]]; then
            log_error "${ecs_service} deployment failed!"
            return 1
        fi

        sleep $interval
        elapsed=$((elapsed + interval))
    done

    log_warn "${ecs_service} deployment timeout - checking health..."

    # Check if running tasks exist
    local running_count
    running_count=$(aws ecs describe-services \
        --cluster "${cluster}" \
        --services "${ecs_service}" \
        --query 'services[0].runningCount' \
        --output text)

    if [[ "${running_count}" -gt 0 ]]; then
        log_info "${ecs_service} has ${running_count} running tasks"
        return 0
    else
        log_error "${ecs_service} has no running tasks"
        return 1
    fi
}

run_health_check() {
    local alb_dns="$1"

    log_step "Running health checks..."

    # Check ALB endpoint
    local health_url="https://${alb_dns}/health"
    local response

    response=$(curl -s -o /dev/null -w "%{http_code}" "${health_url}" || echo "000")

    if [[ "${response}" == "200" ]] || [[ "${response}" == "301" ]] || [[ "${response}" == "302" ]]; then
        log_info "Health check passed (HTTP ${response})"
        return 0
    else
        log_warn "Health check returned HTTP ${response}"
        return 1
    fi
}

# Main execution
ENVIRONMENT="${1:-}"
IMAGE_TAG="${2:-latest}"
shift 2 || shift || true
SERVICES=("${@:-${DEPLOYMENT_ORDER[@]}}")

if [[ -z "${ENVIRONMENT}" ]]; then
    log_error "Environment is required"
    usage
fi

if [[ "${ENVIRONMENT}" != "dev" && "${ENVIRONMENT}" != "prod" ]]; then
    log_error "Invalid environment: ${ENVIRONMENT}"
    usage
fi

CLUSTER=$(get_cluster_name)

log_info "Environment: ${ENVIRONMENT}"
log_info "Cluster: ${CLUSTER}"
log_info "Image tag: ${IMAGE_TAG}"
log_info "Services to deploy: ${SERVICES[*]}"

# Confirm production deployment
if [[ "${ENVIRONMENT}" == "prod" ]]; then
    log_warn "You are about to deploy to PRODUCTION!"
    read -p "Continue? (yes/no): " confirm
    if [[ "${confirm}" != "yes" ]]; then
        log_error "Deployment cancelled"
        exit 1
    fi
fi

# Update each service
FAILED_SERVICES=()
for service in "${SERVICES[@]}"; do
    if ! update_service "${service}" "${CLUSTER}"; then
        FAILED_SERVICES+=("${service}")
    fi
done

# Wait for services to stabilize
log_info "Waiting for services to stabilize..."
for service in "${SERVICES[@]}"; do
    if ! wait_for_service "${service}" "${CLUSTER}"; then
        if [[ ! " ${FAILED_SERVICES[*]} " =~ " ${service} " ]]; then
            FAILED_SERVICES+=("${service}")
        fi
    fi
done

# Get ALB DNS for health check
ALB_DNS=$(aws elbv2 describe-load-balancers \
    --names "${ENVIRONMENT}-shortas-public-alb" \
    --query 'LoadBalancers[0].DNSName' \
    --output text 2>/dev/null || echo "")

if [[ -n "${ALB_DNS}" ]]; then
    run_health_check "${ALB_DNS}" || true
fi

# Summary
echo ""
log_info "========== Deployment Summary =========="
log_info "Environment: ${ENVIRONMENT}"
log_info "Services deployed: ${#SERVICES[@]}"

if [[ ${#FAILED_SERVICES[@]} -gt 0 ]]; then
    log_error "Failed services: ${FAILED_SERVICES[*]}"
    exit 1
else
    log_info "All services deployed successfully!"
fi

if [[ -n "${ALB_DNS}" ]]; then
    echo ""
    log_info "Application URL: https://${ALB_DNS}"
fi
