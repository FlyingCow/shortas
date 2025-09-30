test-click-router: 
	@printf -- "Testing click-router\n\n"
	
	cargo test
	
	@printf -- "DONE: Testing click-router\n\n"

test-unit-click-router:
	@printf -- "Running unit tests for click-router\n\n"
	
	cargo test --lib
	
	@printf -- "DONE: Unit tests for click-router\n\n"

test-integration-click-router:
	@printf -- "Running integration tests for click-router\n\n"
	
	cargo test --test integration
	
	@printf -- "DONE: Integration tests for click-router\n\n"

test-mock-click-router:
	@printf -- "Running mock tests for click-router\n\n"
	
	cargo test --test mock
	
	@printf -- "DONE: Mock tests for click-router\n\n"

test-coverage-click-router:
	@printf -- "Running test coverage for click-router\n\n"
	
	cargo tarpaulin --out Html --output-dir coverage/
	
	@printf -- "DONE: Test coverage for click-router\n\n"

test-performance-click-router:
	@printf -- "Running performance tests for click-router\n\n"
	
	cargo bench
	
	@printf -- "DONE: Performance tests for click-router\n\n"

test-all-click-router: test-unit-click-router test-integration-click-router test-mock-click-router test-coverage-click-router test-performance-click-router
	@printf -- "All tests completed for click-router\n\n"
