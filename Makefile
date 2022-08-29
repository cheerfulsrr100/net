rs_server:
	cargo run --bin server

rs_client:
	cargo run --bin client

cc_cmake:
	cmake -S . -B ./build

cc_build:
	cd build; echo $(abspath $(lastword $(MAKEFILE_LIST))); make unix_rs

cc_exec:
	cd build; echo $(abspath $(lastword $(MAKEFILE_LIST))); ./unix_rs

cc_dy_format:
	clang-format -n -Werror src/*

cc_format:
	clang-format -i src/*

p_tree:
	tree -L 1 -I "build|CMakeFiles|*.cmake|target|*.lock" > treetmp