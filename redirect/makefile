mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

build: build-click-router
build: build-click-router-api

test: test-click-router
test: test-click-router-api

start-local:
	./scripts/local-env-start.sh

install-local:
	./scripts/local-env-install.sh

build-click-router:
	@printf -- "Building click-router\n\n"
	
	cargo build \
		--release \
		--manifest-path=click-router/Cargo.toml \
		--config 'build.target-dir = "target/click-router"'
	
	cp -R -f -v \
		./click-router/config \
		./target/click-router/release

	cp -R -f -v \
		./click-router/src/adapters/uaparser/core/regexes.yaml \
		./target/click-router/release/config/user-agents.yaml

	@printf -- "DONE: Building click-router\n\n"

test-click-router: 

	@printf -- "Testing click-router\n\n"

	cargo test \
		--manifest-path=click-router/Cargo.toml
	
	@printf -- "DONE: Testing click-router\n\n"


build-click-router-api:
	@printf -- "Building click-router-api\n\n"
	
	cargo build \
		--release \
		--manifest-path=click-router-api/Cargo.toml \
		--config 'build.target-dir = "target/click-router-api"'
	
	cp -R -f -v \
		./click-router-api/config \
		./target/click-router-api/release

	@printf -- "DONE: Building click-router-api\n\n"

test-click-router-api: 

	@printf -- "Testing click-router-api\n\n"

	cargo test \
		--manifest-path=click-router-api/Cargo.toml
	
	@printf -- "DONE: Testing click-router-api\n\n"