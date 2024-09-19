TARGET := riscv64imac-unknown-none-elf
CC := riscv64-unknown-elf-gcc

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
	cargo build --target riscv64imac-unknown-none-elf --example demo

target/riscv64imac-unknown-none-elf/debug/examples/exec_callee:
	cargo build --target riscv64imac-unknown-none-elf --example exec_callee

target/riscv64imac-unknown-none-elf/debug/examples/exec_caller_by_code_hash:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="dummy-atomic" --example exec_caller_by_code_hash

target/riscv64imac-unknown-none-elf/debug/examples/exec_caller:
	cargo build --target riscv64imac-unknown-none-elf --example exec_caller

target/riscv64imac-unknown-none-elf/debug/examples/spawn_callee:
	cargo build --target riscv64imac-unknown-none-elf --example spawn_callee

target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller_by_code_hash:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="dummy-atomic" --example spawn_caller_by_code_hash

target/riscv64imac-unknown-none-elf/debug/examples/spawn_caller:
	cargo build --target riscv64imac-unknown-none-elf --example spawn_caller

target/riscv64imac-unknown-none-elf/debug/examples/std_test:
	RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf --features="dlopen-c,dummy-atomic,log" --example std_test

default: integration

publish-crate:
	cross publish -p ckb-std

publish: publish-crate

clean:
	cross clean && make -C test clean

test-shared-lib:
	make -C test/shared-lib all-via-docker

integration: check

test:
	make -C test test

check:
	cross check --target ${TARGET} --examples

.PHONY: test check
