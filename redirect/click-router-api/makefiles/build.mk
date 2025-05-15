build-click-router-api: 
	@printf -- "Building click-router-api\n\n"
	
	cargo build
	
	@printf -- "DONE: Building click-router-api\n\n"

clean-click-router-api:
	cargo clean