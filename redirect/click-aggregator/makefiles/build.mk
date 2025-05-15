build-click-aggregator: 
	@printf -- "Building click-aggregator\n\n"
	
	cargo build
	
	@printf -- "DONE: Building click-aggregator\n\n"

clean-click-aggregator:
	cargo clean