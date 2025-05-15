build-click-router: 
	@printf -- "Building click-router\n\n"
	
	cargo build
	
	@printf -- "DONE: Building click-router\n\n"

clean-click-router:
	cargo clean