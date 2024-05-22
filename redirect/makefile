mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

start-local:
	./scripts/local-env-start.sh

install-local:
	./scripts/local-env-install.sh

build-click-router:
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