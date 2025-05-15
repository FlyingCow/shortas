release-click-router-api: 
	@printf -- "Building release click-router-api\n\n"
	
		cargo build \
		--release \
		--manifest-path=Cargo.toml \
		--config 'build.target-dir = "target"'
	
	cp -R -f -v \
		./config \
		./target/release

	@printf -- "DONE: Building release click-router-api\n\n"