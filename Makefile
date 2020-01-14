TARGET := riscv64imac-unknown-none-elf

default: integration

integration: check test

test:
	make -C test test

check:
	cargo check --target ${TARGET} --examples

.PHONY: test check
