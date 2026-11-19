ROOT_MANIFEST := Cargo.toml
REDIRECT_MANIFEST := redirect/Cargo.toml
UI_DIR := ui/dashboard
COMPOSE := docker compose -f redirect/docker-compose.yml

# Crates in the redirect workspace; everything else lives in the root workspace
# (api, workers/*, redirect/shortas-common).
REDIRECT_PKGS := click-router click-router-api click-tracker click-aggregator click-aggregator-api

# Manifest that owns package $(1)
manifest_for = $(if $(filter $(1),$(REDIRECT_PKGS)),$(REDIRECT_MANIFEST),$(ROOT_MANIFEST))

# Run `cargo $(1)` against both workspaces
define both
	cargo $(1) --manifest-path $(ROOT_MANIFEST) $(2)
	cargo $(1) --manifest-path $(REDIRECT_MANIFEST) $(2)
endef

# ── Build ─────────────────────────────────────────────────

.PHONY: build build-rust build-ui
build: build-rust

build-rust:
	$(call both,build,--workspace)

build-ui:
	npm --prefix $(UI_DIR) install
	npm --prefix $(UI_DIR) run build

build-%:
	cargo build --manifest-path $(call manifest_for,$*) -p $*

# ── Release ───────────────────────────────────────────────

.PHONY: release
release:
	$(call both,build,--release --workspace)

release-%:
	cargo build --release --manifest-path $(call manifest_for,$*) -p $*

# ── Test ──────────────────────────────────────────────────

.PHONY: test test-rust test-ui
test: test-rust

test-rust:
	$(call both,test,--workspace)

test-ui:
	npm --prefix $(UI_DIR) test

test-%:
	cargo test --manifest-path $(call manifest_for,$*) -p $*

# ── Lint / Check ──────────────────────────────────────────

.PHONY: check clippy fmt
check:
	$(call both,check,--workspace)

clippy:
	$(call both,clippy,--workspace -- -D warnings)

fmt:
	$(call both,fmt,--all)

# ── Bench ─────────────────────────────────────────────────

.PHONY: bench
bench:
	$(call both,bench,--workspace)

bench-%:
	cargo bench --manifest-path $(call manifest_for,$*) -p $*

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
	$(call both,clean,)
