release-click-tracker: 
	@printf -- "Building release click-tracker\n\n"
	
	cargo build \
		--release \
		--manifest-path=Cargo.toml \
		--config 'build.target-dir = "target"'
	
	cp -R -f -v \
		./config \
		./target/release

	mkdir -p ./target/release/data/


	mkdir -p ./target/release/data/ua-parser/

	cp -R -f -v \
		../data/ua-parser/regexes.yaml \
		./target/release/data/ua-parser/regexes.yaml


	mkdir -p ./target/release/data/geo-ip/
	cp -R -f -v \
		../data/geo-ip/GeoLite2-Country.mmdb \
		./target/release/data/geo-ip/GeoLite2-Country.mmdb

	@printf -- "DONE: Building release click-tracker\n\n"