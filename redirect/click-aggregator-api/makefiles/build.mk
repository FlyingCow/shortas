build-click-aggregator-api: 
	@printf -- "Building click-aggregator-api\n\n"
	
	cargo build
	
	@printf -- "DONE: Building click-aggregator-api\n\n"

clean-click-aggregator-api:
	cargo clean