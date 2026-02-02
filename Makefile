RUST_DIR := redirect
API_DIR := api
UI_DIR := ui/dashboard
COMPOSE := docker compose -f $(RUST_DIR)/docker-compose.yml

SERVICES := click-router click-router-api click-tracker click-aggregator click-aggregator-api

# ── Build ─────────────────────────────────────────────────

.PHONY: build build-api build-ui build-rust
build: build-rust

build-rust:
	cargo build --manifest-path $(RUST_DIR)/Cargo.toml

build-api:
	dotnet build $(API_DIR)/api.sln

build-ui:
	npm --prefix $(UI_DIR) install
	npm --prefix $(UI_DIR) run build

build-%:
	cargo build --manifest-path $(RUST_DIR)/Cargo.toml -p $*

# ── Release ───────────────────────────────────────────────

.PHONY: release release-api
release:
	cargo build --release --manifest-path $(RUST_DIR)/Cargo.toml

release-api:
	dotnet publish $(API_DIR)/api.sln -c Release

release-%:
	cargo build --release --manifest-path $(RUST_DIR)/Cargo.toml -p $*

# ── Test ──────────────────────────────────────────────────

.PHONY: test test-api test-ui test-rust
test: test-rust

test-rust:
	cargo test --manifest-path $(RUST_DIR)/Cargo.toml

test-api:
	dotnet test $(API_DIR)/api.sln

test-ui:
	npm --prefix $(UI_DIR) test

test-%:
	cargo test --manifest-path $(RUST_DIR)/Cargo.toml -p $*

# ── Lint / Check ──────────────────────────────────────────

.PHONY: check clippy fmt
check:
	cargo check --manifest-path $(RUST_DIR)/Cargo.toml

clippy:
	cargo clippy --manifest-path $(RUST_DIR)/Cargo.toml -- -D warnings

fmt:
	cargo fmt --manifest-path $(RUST_DIR)/Cargo.toml --all

# ── Bench ─────────────────────────────────────────────────

.PHONY: bench
bench:
	cargo bench --manifest-path $(RUST_DIR)/Cargo.toml

bench-%:
	cargo bench --manifest-path $(RUST_DIR)/Cargo.toml -p $*

# ── Docker ────────────────────────────────────────────────

.PHONY: up down logs ps
up:
	$(COMPOSE) up -d

down:
	$(COMPOSE) down

logs:
	$(COMPOSE) logs -f

ps:
	$(COMPOSE) ps

# ── Clean ─────────────────────────────────────────────────

.PHONY: clean
clean:
	cargo clean --manifest-path $(RUST_DIR)/Cargo.toml
