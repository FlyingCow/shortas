#!/bin/bash
set -euo pipefail

# Build and Push Docker Images to ECR
# Usage: ./build-push-images.sh <environment> [service1 service2 ...]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../../.."
REDIRECT_DIR="${PROJECT_ROOT}/redirect"

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

# All services
ALL_SERVICES=(
    "click-router"
    "click-router-api"
    "click-tracker"
    "click-aggregator"
    "click-aggregator-api"
    "domain-verifier"
    "route-verifier"
    "route-icon-worker"
    "cert-bot"
    "management-api"
    "dashboard"
    "landing"
    "pages"
)

usage() {
    echo "Usage: $0 <environment> [service1 service2 ...]"
    echo ""
    echo "Arguments:"
    echo "  environment    Environment (dev|prod)"
    echo "  services       Optional list of services to build (default: all)"
    echo ""
    echo "Available services:"
    printf "  %s\n" "${ALL_SERVICES[@]}"
    echo ""
    echo "Examples:"
    echo "  $0 dev                      # Build all services for dev"
    echo "  $0 prod click-router        # Build only click-router for prod"
    exit 1
}

get_ecr_url() {
    local account_id region
    account_id=$(aws sts get-caller-identity --query Account --output text)
    region=$(aws configure get region || echo "us-east-1")
    echo "${account_id}.dkr.ecr.${region}.amazonaws.com"
}

ecr_login() {
    log_info "Logging into ECR..."
    local ecr_url region
    region=$(aws configure get region || echo "us-east-1")
    ecr_url=$(get_ecr_url)
    aws ecr get-login-password --region "${region}" | docker login --username AWS --password-stdin "${ecr_url}"
}

get_image_tag() {
    # Use git commit SHA as tag, or 'latest' if not in git repo
    if git rev-parse --git-dir > /dev/null 2>&1; then
        git rev-parse --short HEAD
    else
        echo "latest"
    fi
}

build_service() {
    local service="$1"
    local ecr_url="$2"
    local tag="$3"
    local repo="${ENVIRONMENT}-shortas/${service}"
    local full_image="${ecr_url}/${repo}"

    log_step "Building ${service}..."

    # Every service's Dockerfile is self-contained (builds exactly one binary/app,
    # no SERVICE build-arg) and its COPY paths are written relative to the repo
    # root, so the build context is the repo root for all of them except the two
    # Node/nginx frontends, which are self-contained within their own directory.
    local dockerfile=""
    local context="${PROJECT_ROOT}"

    case "${service}" in
        click-router|click-router-api|click-tracker|click-aggregator|click-aggregator-api|domain-verifier|route-verifier|route-icon-worker|cert-bot)
            dockerfile="${REDIRECT_DIR}/${service}/Dockerfile"
            ;;
        management-api)
            dockerfile="${PROJECT_ROOT}/management-api/Dockerfile"
            ;;
        dashboard)
            dockerfile="${PROJECT_ROOT}/ui/dashboard/Dockerfile"
            context="${PROJECT_ROOT}/ui/dashboard"
            ;;
        landing)
            dockerfile="${PROJECT_ROOT}/ui/landing/Dockerfile"
            context="${PROJECT_ROOT}/ui/landing"
            ;;
        pages)
            dockerfile="${REDIRECT_DIR}/infra/pages/Dockerfile"
            ;;
        *)
            log_error "Unknown service: ${service}"
            return 1
            ;;
    esac

    # Build the image
    docker build \
        -f "${dockerfile}" \
        -t "${full_image}:${tag}" \
        -t "${full_image}:latest" \
        "${context}"

    log_info "Built ${full_image}:${tag}"
}

push_service() {
    local service="$1"
    local ecr_url="$2"
    local tag="$3"
    local repo="${ENVIRONMENT}-shortas/${service}"
    local full_image="${ecr_url}/${repo}"

    log_step "Pushing ${service}..."

    # Ensure repository exists
    aws ecr describe-repositories --repository-names "${repo}" > /dev/null 2>&1 || \
        aws ecr create-repository --repository-name "${repo}" --image-scanning-configuration scanOnPush=true

    docker push "${full_image}:${tag}"
    docker push "${full_image}:latest"

    log_info "Pushed ${full_image}:${tag}"
}

# Main execution
ENVIRONMENT="${1:-}"
shift || true
SERVICES=("${@:-${ALL_SERVICES[@]}}")

if [[ -z "${ENVIRONMENT}" ]]; then
    log_error "Environment is required"
    usage
fi

if [[ "${ENVIRONMENT}" != "dev" && "${ENVIRONMENT}" != "prod" ]]; then
    log_error "Invalid environment: ${ENVIRONMENT}"
    usage
fi

# Get ECR URL and tag
ECR_URL=$(get_ecr_url)
IMAGE_TAG=$(get_image_tag)

log_info "Environment: ${ENVIRONMENT}"
log_info "ECR URL: ${ECR_URL}"
log_info "Image tag: ${IMAGE_TAG}"
log_info "Services: ${SERVICES[*]}"

# Login to ECR
ecr_login

# Build and push each service
for service in "${SERVICES[@]}"; do
    build_service "${service}" "${ECR_URL}" "${IMAGE_TAG}"
    push_service "${service}" "${ECR_URL}" "${IMAGE_TAG}"
done

log_info "All images built and pushed successfully!"
echo ""
log_info "To deploy, run:"
echo "  ./deploy-services.sh ${ENVIRONMENT} ${IMAGE_TAG}"
