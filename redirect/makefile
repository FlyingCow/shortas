
# Shortas Redirect System - Build System
# ======================================

# Service directories
SERVICES := click-router click-router-api click-tracker click-aggregator click-aggregator-api
DOCKER_SERVICES := click-router click-router-api click-tracker click-aggregator click-aggregator-api

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
RESET := \033[0m

# Default target
.DEFAULT_GOAL := help

# Include service makefiles
include click-router/makefile
include click-router-api/makefile
include click-tracker/makefile
include click-aggregator/makefile
include click-aggregator-api/makefile

# =============================================================================
# HELP & INFORMATION
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Shortas Redirect System - Build Commands$(RESET)"
	@echo "$(CYAN)========================================$(RESET)"
	@echo ""
	@echo "$(YELLOW)Development Commands:$(RESET)"
	@echo "  $(GREEN)dev-setup$(RESET)          - Complete development environment setup"
	@echo "  $(GREEN)dev-start$(RESET)         - Start all services in development mode"
	@echo "  $(GREEN)dev-stop$(RESET)          - Stop all development services"
	@echo "  $(GREEN)dev-restart$(RESET)      - Restart all development services"
	@echo ""
	@echo "$(YELLOW)Build Commands:$(RESET)"
	@echo "  $(GREEN)build$(RESET)             - Build all services (debug)"
	@echo "  $(GREEN)build-release$(RESET)     - Build all services (release)"
	@echo "  $(GREEN)build-docker$(RESET)      - Build all Docker images"
	@echo "  $(GREEN)build-$(SERVICE)$(RESET)  - Build specific service"
	@echo ""
	@echo "$(YELLOW)Testing Commands:$(RESET)"
	@echo "  $(GREEN)test$(RESET)              - Run all tests"
	@echo "  $(GREEN)test-watch$(RESET)       - Run tests in watch mode"
	@echo "  $(GREEN)test-coverage$(RESET)    - Run tests with coverage"
	@echo "  $(GREEN)test-$(SERVICE)$(RESET)  - Test specific service"
	@echo ""
	@echo "$(YELLOW)Infrastructure Commands:$(RESET)"
	@echo "  $(GREEN)infra-start$(RESET)      - Start infrastructure (databases, queues)"
	@echo "  $(GREEN)infra-stop$(RESET)       - Stop infrastructure"
	@echo "  $(GREEN)infra-reset$(RESET)      - Reset infrastructure (clean data)"
	@echo ""
	@echo "$(YELLOW)Deployment Commands:$(RESET)"
	@echo "  $(GREEN)deploy-local$(RESET)     - Deploy to local environment"
	@echo "  $(GREEN)deploy-docker$(RESET)    - Deploy using Docker Compose"
	@echo "  $(GREEN)deploy-aws$(RESET)       - Deploy to AWS (requires terraform)"
	@echo ""
	@echo "$(YELLOW)Health & Monitoring Commands:$(RESET)"
	@echo "  $(GREEN)health-check$(RESET)     - Check health of all services"
	@echo "  $(GREEN)health-check-infra$(RESET) - Check infrastructure health"
	@echo "  $(GREEN)health-check-apps$(RESET) - Check application health"
	@echo "  $(GREEN)validate$(RESET)        - Validate system configuration"
	@echo ""
	@echo "$(YELLOW)Utility Commands:$(RESET)"
	@echo "  $(GREEN)clean$(RESET)            - Clean all build artifacts"
	@echo "  $(GREEN)lint$(RESET)             - Run linters on all services"
	@echo "  $(GREEN)format$(RESET)           - Format all code"
	@echo "  $(GREEN)check$(RESET)            - Run all checks (lint, test, build)"
	@echo "  $(GREEN)logs$(RESET)             - Show logs for all services"
	@echo "  $(GREEN)logs-$(SERVICE)$(RESET)   - Show logs for specific service"
	@echo ""
	@echo "$(YELLOW)Service-specific Commands:$(RESET)"
	@for service in $(SERVICES); do \
		echo "  $(GREEN)$$service$(RESET)"; \
	done
	@echo ""
	@echo "$(CYAN)Examples:$(RESET)"
	@echo "  make dev-setup                    # Complete setup"
	@echo "  make build-click-router          # Build only router"
	@echo "  make test-watch                   # Watch mode testing"
	@echo "  make deploy-docker               # Docker deployment"

# =============================================================================
# DEVELOPMENT COMMANDS
# =============================================================================

.PHONY: dev-setup
dev-setup: ## Complete development environment setup
	@echo "$(BLUE)Setting up development environment...$(RESET)"
	@echo "$(YELLOW)1. Installing system dependencies...$(RESET)"
	@./scripts/local-env-install.sh
	@echo "$(YELLOW)2. Starting infrastructure...$(RESET)"
	@$(MAKE) infra-start
	@echo "$(YELLOW)3. Building all services...$(RESET)"
	@$(MAKE) build
	@echo "$(YELLOW)4. Running tests...$(RESET)"
	@$(MAKE) test
	@echo "$(GREEN)Development environment setup complete!$(RESET)"

.PHONY: dev-start
dev-start: ## Start all services in development mode
	@echo "$(BLUE)Starting development services...$(RESET)"
	@echo "$(YELLOW)Starting infrastructure...$(RESET)"
	@$(MAKE) infra-start
	@echo "$(YELLOW)Starting services...$(RESET)"
	@echo "$(CYAN)Use 'make dev-stop' to stop all services$(RESET)"

.PHONY: dev-stop
dev-stop: ## Stop all development services
	@echo "$(BLUE)Stopping development services...$(RESET)"
	@$(MAKE) infra-stop

.PHONY: dev-restart
dev-restart: dev-stop dev-start ## Restart all development services

# =============================================================================
# BUILD COMMANDS
# =============================================================================

.PHONY: build
build: ## Build all services (debug)
	@echo "$(BLUE)Building all services...$(RESET)"
	@for service in $(SERVICES); do \
		echo "$(YELLOW)Building $$service...$(RESET)"; \
		$(MAKE) build-$$service; \
	done
	@echo "$(GREEN)All services built successfully!$(RESET)"

.PHONY: build-release
build-release: ## Build all services (release)
	@echo "$(BLUE)Building all services (release)...$(RESET)"
	@for service in $(SERVICES); do \
		echo "$(YELLOW)Building $$service (release)...$(RESET)"; \
		$(MAKE) release-$$service; \
	done
	@echo "$(GREEN)All services built successfully (release)!$(RESET)"

.PHONY: build-docker
build-docker: ## Build all Docker images
	@echo "$(BLUE)Building Docker images...$(RESET)"
	@for service in $(DOCKER_SERVICES); do \
		echo "$(YELLOW)Building Docker image for $$service...$(RESET)"; \
		docker build -t shortas/$$service:latest ./$$service/; \
	done
	@echo "$(GREEN)All Docker images built successfully!$(RESET)"

# =============================================================================
# TESTING COMMANDS
# =============================================================================

.PHONY: test
test: ## Run all tests
	@echo "$(BLUE)Running all tests...$(RESET)"
	@for service in $(SERVICES); do \
		echo "$(YELLOW)Testing $$service...$(RESET)"; \
		$(MAKE) test-$$service; \
	done
	@echo "$(GREEN)All tests completed!$(RESET)"

.PHONY: test-watch
test-watch: ## Run tests in watch mode
	@echo "$(BLUE)Running tests in watch mode...$(RESET)"
	@echo "$(CYAN)Press Ctrl+C to stop$(RESET)"
	@cargo watch -x test

.PHONY: test-coverage
test-coverage: ## Run tests with coverage
	@echo "$(BLUE)Running tests with coverage...$(RESET)"
	@cargo install cargo-tarpaulin
	@cargo tarpaulin --out Html --output-dir coverage/
	@echo "$(GREEN)Coverage report generated in coverage/$(RESET)"

# =============================================================================
# INFRASTRUCTURE COMMANDS
# =============================================================================

.PHONY: infra-start
infra-start: ## Start infrastructure (databases, queues)
	@echo "$(BLUE)Starting infrastructure...$(RESET)"
	@echo "$(YELLOW)Choose infrastructure type:$(RESET)"
	@echo "  $(GREEN)make infra-start-custom$(RESET)  - Custom infrastructure (MongoDB, ClickHouse, Redis, Fluvio)"
	@echo "  $(GREEN)make infra-start-aws$(RESET)    - AWS LocalStack infrastructure"
	@echo "$(CYAN)Defaulting to custom infrastructure...$(RESET)"
	@$(MAKE) infra-start-custom

.PHONY: infra-start-custom
infra-start-custom: ## Start custom infrastructure
	@echo "$(BLUE)Starting custom infrastructure...$(RESET)"
	@./infra/custom/local-env-start.sh

.PHONY: infra-start-aws
infra-start-aws: ## Start AWS LocalStack infrastructure
	@echo "$(BLUE)Starting AWS LocalStack infrastructure...$(RESET)"
	@./infra/aws/local-env-start.sh

.PHONY: infra-stop
infra-stop: ## Stop infrastructure
	@echo "$(BLUE)Stopping infrastructure...$(RESET)"
	@docker-compose -f infra/custom/docker-services.local.yml down
	@docker-compose -f infra/aws/docker-services.local.yml down
	@echo "$(GREEN)Infrastructure stopped!$(RESET)"

.PHONY: infra-reset
infra-reset: ## Reset infrastructure (clean data)
	@echo "$(BLUE)Resetting infrastructure...$(RESET)"
	@$(MAKE) infra-stop
	@docker system prune -f
	@docker volume prune -f
	@echo "$(GREEN)Infrastructure reset complete!$(RESET)"

# =============================================================================
# DEPLOYMENT COMMANDS
# =============================================================================

.PHONY: deploy-local
deploy-local: build-release ## Deploy to local environment
	@echo "$(BLUE)Deploying to local environment...$(RESET)"
	@echo "$(GREEN)Local deployment complete!$(RESET)"

.PHONY: deploy-docker
deploy-docker: build-docker ## Deploy using Docker Compose
	@echo "$(BLUE)Deploying with Docker Compose...$(RESET)"
	@docker-compose -f docker-compose.yml up -d
	@echo "$(GREEN)Docker deployment complete!$(RESET)"

.PHONY: deploy-aws
deploy-aws: ## Deploy to AWS (requires terraform)
	@echo "$(BLUE)Deploying to AWS...$(RESET)"
	@cd infra/aws/terraform && terraform init && terraform apply
	@echo "$(GREEN)AWS deployment complete!$(RESET)"

# =============================================================================
# HEALTH CHECK COMMANDS
# =============================================================================

.PHONY: health-check
health-check: ## Check health of all services
	@echo "$(BLUE)Checking service health...$(RESET)"
	@echo "$(YELLOW)Checking infrastructure services...$(RESET)"
	@$(MAKE) health-check-infra
	@echo "$(YELLOW)Checking application services...$(RESET)"
	@$(MAKE) health-check-apps
	@echo "$(GREEN)All health checks passed!$(RESET)"

.PHONY: health-check-infra
health-check-infra: ## Check infrastructure health
	@echo "$(BLUE)Checking infrastructure health...$(RESET)"
	@echo "$(YELLOW)MongoDB:$(RESET)"
	@docker exec shortas-mongo mongosh --eval "db.adminCommand('ping')" || echo "$(RED)MongoDB not healthy$(RESET)"
	@echo "$(YELLOW)ClickHouse:$(RESET)"
	@curl -f http://localhost:8123/ping || echo "$(RED)ClickHouse not healthy$(RESET)"
	@echo "$(YELLOW)Redis:$(RESET)"
	@docker exec shortas-redis redis-cli ping || echo "$(RED)Redis not healthy$(RESET)"
	@echo "$(YELLOW)Fluvio:$(RESET)"
	@docker exec shortas-fluvio-sc fluvio version || echo "$(RED)Fluvio not healthy$(RESET)"

.PHONY: health-check-apps
health-check-apps: ## Check application health
	@echo "$(BLUE)Checking application health...$(RESET)"
	@echo "$(YELLOW)Click Router:$(RESET)"
	@curl -f http://localhost:8080/health || echo "$(RED)Click Router not healthy$(RESET)"
	@echo "$(YELLOW)Click Router API:$(RESET)"
	@curl -f http://localhost:8081/health || echo "$(RED)Click Router API not healthy$(RESET)"
	@echo "$(YELLOW)Click Aggregator API:$(RESET)"
	@curl -f http://localhost:8082/health || echo "$(RED)Click Aggregator API not healthy$(RESET)"

.PHONY: validate
validate: ## Validate system configuration and dependencies
	@echo "$(BLUE)Validating system...$(RESET)"
	@echo "$(YELLOW)Checking Rust installation...$(RESET)"
	@rustc --version || (echo "$(RED)Rust not installed$(RESET)" && exit 1)
	@echo "$(YELLOW)Checking Cargo installation...$(RESET)"
	@cargo --version || (echo "$(RED)Cargo not installed$(RESET)" && exit 1)
	@echo "$(YELLOW)Checking Docker installation...$(RESET)"
	@docker --version || (echo "$(RED)Docker not installed$(RESET)" && exit 1)
	@echo "$(YELLOW)Checking Docker Compose installation...$(RESET)"
	@docker-compose --version || (echo "$(RED)Docker Compose not installed$(RESET)" && exit 1)
	@echo "$(GREEN)System validation passed!$(RESET)"

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

.PHONY: clean
clean: ## Clean all build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(RESET)"
	@for service in $(SERVICES); do \
		echo "$(YELLOW)Cleaning $$service...$(RESET)"; \
		$(MAKE) clean-$$service; \
	done
	@cargo clean
	@echo "$(GREEN)Clean complete!$(RESET)"

.PHONY: lint
lint: ## Run linters on all services
	@echo "$(BLUE)Running linters...$(RESET)"
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Linting complete!$(RESET)"

.PHONY: format
format: ## Format all code
	@echo "$(BLUE)Formatting code...$(RESET)"
	@cargo fmt --all
	@echo "$(GREEN)Formatting complete!$(RESET)"

.PHONY: check
check: lint test build ## Run all checks (lint, test, build)
	@echo "$(GREEN)All checks passed!$(RESET)"

.PHONY: logs
logs: ## Show logs for all services
	@echo "$(BLUE)Showing service logs...$(RESET)"
	@docker-compose logs -f

.PHONY: logs-router
logs-router: ## Show logs for click-router
	@echo "$(BLUE)Showing click-router logs...$(RESET)"
	@docker-compose logs -f click-router

.PHONY: logs-tracker
logs-tracker: ## Show logs for click-tracker
	@echo "$(BLUE)Showing click-tracker logs...$(RESET)"
	@docker-compose logs -f click-tracker

.PHONY: logs-aggregator
logs-aggregator: ## Show logs for click-aggregator
	@echo "$(BLUE)Showing click-aggregator logs...$(RESET)"
	@docker-compose logs -f click-aggregator

# =============================================================================
# SERVICE-SPECIFIC TARGETS
# =============================================================================

# Build targets for each service
build-click-router: build-click-router
build-click-router-api: build-click-router-api
build-click-tracker: build-click-tracker
build-click-aggregator: build-click-aggregator
build-click-aggregator-api: build-click-aggregator-api

# Test targets for each service
test-click-router: test-click-router
test-click-router-api: test-click-router-api
test-click-tracker: test-click-tracker
test-click-aggregator: test-click-aggregator
test-click-aggregator-api: test-click-aggregator-api

# Release targets for each service
release-click-router: release-click-router
release-click-router-api: release-click-router-api
release-click-tracker: release-click-tracker
release-click-aggregator: release-click-aggregator
release-click-aggregator-api: release-click-aggregator-api

# Clean targets for each service
clean-click-router: clean-click-router
clean-click-router-api: clean-click-router-api
clean-click-tracker: clean-click-tracker
clean-click-aggregator: clean-click-aggregator
clean-click-aggregator-api: clean-click-aggregator-api

# =============================================================================
# LEGACY TARGETS (for backward compatibility)
# =============================================================================

start-aws-local: infra-start-aws
start-custom-local: infra-start-custom
install-local: dev-setup
