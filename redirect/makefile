
include click-router/makefile
include click-router-api/makefile
include click-tracker/makefile
include click-aggregator/makefile
include click-aggregator-api/makefile

build: build-click-router
build: build-click-router-api
build: build-click-tracker
build: build-click-aggregator
build: build-click-aggregator-api

test: test-click-router
test: test-click-router-api
test: test-click-tracker
test: test-click-aggregator
test: test-click-aggregator-api

bench: bench-click-router
bench: bench-click-router-api
bench: bench-click-tracker
bench: bench-click-aggregator
bench: bench-click-aggregator-api

clean: clean-click-router
clean: clean-click-router-api
clean: clean-click-tracker
clean: clean-click-aggregator
clean: clean-click-aggregator-api

start-aws-local:
	./infra/aws/local-env-start.sh
	
start-custom-local:
	./infra/custom/local-env-start.sh

install-local:
	./scripts/local-env-install.sh
