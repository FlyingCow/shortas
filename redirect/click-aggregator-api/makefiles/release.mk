release-click-aggregator-api: 
	@printf -- "Building release click-aggregator-api\n\n"
	
		cargo build \
		--release \
		--manifest-path=Cargo.toml \
		--config 'build.target-dir = "target"'
	
	cp -R -f -v \
		./config \
		./target/release

	@printf -- "DONE: Building release click-aggregator-api\n\n"