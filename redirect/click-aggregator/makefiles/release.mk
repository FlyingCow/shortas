release-click-aggregator: 
	@printf -- "Building release click-aggregator\n\n"
	
	cargo build \
		--release \
		--manifest-path=Cargo.toml \
		--config 'build.target-dir = "target"'
	
	cp -R -f -v \
		./config \
		./target/release

	@printf -- "DONE: Building release click-aggregator\n\n"