#!/bin/bash
set -euo pipefail

# Deploy Shortas Infrastructure to AWS
# Usage: ./deploy-infra.sh <environment> [plan|apply|destroy]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TERRAFORM_DIR="${SCRIPT_DIR}/../terraform"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

usage() {
    echo "Usage: $0 <environment> [action]"
    echo ""
    echo "Arguments:"
    echo "  environment    Environment to deploy (dev|prod)"
    echo "  action         Terraform action (plan|apply|destroy) - default: plan"
    echo ""
    echo "Examples:"
    echo "  $0 dev plan        # Preview changes for dev environment"
    echo "  $0 dev apply       # Apply changes to dev environment"
    echo "  $0 prod plan       # Preview changes for prod environment"
    exit 1
}

check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check Terraform
    if ! command -v terraform &> /dev/null; then
        log_error "Terraform is not installed. Please install Terraform >= 1.5.0"
        exit 1
    fi

    TERRAFORM_VERSION=$(terraform version -json | jq -r '.terraform_version')
    log_info "Terraform version: ${TERRAFORM_VERSION}"

    # Check AWS CLI
    if ! command -v aws &> /dev/null; then
        log_error "AWS CLI is not installed. Please install AWS CLI v2"
        exit 1
    fi

    # Check AWS credentials
    if ! aws sts get-caller-identity &> /dev/null; then
        log_error "AWS credentials not configured. Please run 'aws configure' or set AWS_PROFILE"
        exit 1
    fi

    AWS_ACCOUNT=$(aws sts get-caller-identity --query Account --output text)
    AWS_REGION=$(aws configure get region || echo "us-east-1")
    log_info "AWS Account: ${AWS_ACCOUNT}"
    log_info "AWS Region: ${AWS_REGION}"
}

init_terraform() {
    local env_dir="${TERRAFORM_DIR}/environments/${ENVIRONMENT}"

    log_info "Initializing Terraform for ${ENVIRONMENT}..."
    cd "${env_dir}"

    terraform init -upgrade
}

run_terraform() {
    local env_dir="${TERRAFORM_DIR}/environments/${ENVIRONMENT}"
    cd "${env_dir}"

    case "${ACTION}" in
        plan)
            log_info "Running Terraform plan for ${ENVIRONMENT}..."
            terraform plan -out=tfplan
            ;;
        apply)
            log_info "Applying Terraform changes for ${ENVIRONMENT}..."
            if [[ -f tfplan ]]; then
                terraform apply tfplan
            else
                terraform apply -auto-approve
            fi
            ;;
        destroy)
            if [[ "${ENVIRONMENT}" == "prod" ]]; then
                log_warn "You are about to destroy PRODUCTION infrastructure!"
                read -p "Type 'destroy-prod' to confirm: " confirm
                if [[ "${confirm}" != "destroy-prod" ]]; then
                    log_error "Destroy cancelled"
                    exit 1
                fi
            fi
            log_warn "Destroying ${ENVIRONMENT} infrastructure..."
            terraform destroy
            ;;
        output)
            log_info "Terraform outputs for ${ENVIRONMENT}:"
            terraform output
            ;;
        *)
            log_error "Unknown action: ${ACTION}"
            usage
            ;;
    esac
}

# Main execution
ENVIRONMENT="${1:-}"
ACTION="${2:-plan}"

if [[ -z "${ENVIRONMENT}" ]]; then
    log_error "Environment is required"
    usage
fi

if [[ "${ENVIRONMENT}" != "dev" && "${ENVIRONMENT}" != "prod" ]]; then
    log_error "Invalid environment: ${ENVIRONMENT}. Must be 'dev' or 'prod'"
    usage
fi

check_prerequisites
init_terraform
run_terraform

log_info "Done!"
