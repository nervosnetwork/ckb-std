all: \
	target/riscv64imac-unknown-none-elf/debug/examples/demo \
	target/riscv64imac-unknown-none-elf/debug/examples/exec_callee \
	target/riscv64imac-unknown-none-elf/debug/examples/exec_caller_by_code_hash \
	target/riscv64imac-unknown-none-elf/debug/examples/exec_caller \
	target/riscv64imac-unknown-none-elf/debug/examples/spawn_callee \
	target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller_by_code_hash \
	target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller \
	target/riscv64imac-unknown-none-elf/debug/examples/std_test

target/riscv64imac-unknown-none-elf/debug/examples/demo:
	cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang" --example demo

target/riscv64imac-unknown-none-elf/debug/examples/exec_callee:
	cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang" --example exec_callee

target/riscv64imac-unknown-none-elf/debug/examples/exec_caller_by_code_hash:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang,dummy-atomic" --example exec_caller_by_code_hash

target/riscv64imac-unknown-none-elf/debug/examples/exec_caller:
	cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang" --example exec_caller

target/riscv64imac-unknown-none-elf/debug/examples/spawn_callee:
	cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang" --example spawn_callee

target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller_by_code_hash:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang,dummy-atomic" --example spawn_caller_by_code_hash

target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller:
	cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang" --example spawn_caller

target/riscv64imac-unknown-none-elf/debug/examples/std_test:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="build-with-clang,dlopen-c,dummy-atomic,log" --example std_test

test-shared-lib:
	make -C test/shared-lib all-via-docker

test:
	make -C test test

.PHONY: \
	test-shared-lib \
	test
