mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

include ${mkfile_dir}/makefiles/build.mk
include ${mkfile_dir}/makefiles/test.mk
include ${mkfile_dir}/makefiles/bench.mk
include ${mkfile_dir}/makefiles/release.mk