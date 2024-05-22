mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

build: build-redirect
test: test-redirect

build-redirect:
		cd ./redirect && $(MAKE) build
		
test-redirect:
		cd ./redirect && $(MAKE) test-click-router