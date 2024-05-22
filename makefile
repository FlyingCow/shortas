mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

build: update-git-submodules
build: build-redirect

test: test-redirect

update-git-submodules:
		git submodule update

build-redirect:
		cd ./redirect && $(MAKE) build
		
test-redirect:
		cd ./redirect && $(MAKE) test-click-router