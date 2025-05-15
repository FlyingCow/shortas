build-click-tracker: 
	@printf -- "Building click-tracker\n\n"
	
	cargo build
	
	@printf -- "DONE: Building click-tracker\n\n"

clean-click-tracker:
	cargo clean