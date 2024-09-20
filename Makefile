TARGET := riscv64imac-unknown-none-elf
CC := riscv64-unknown-elf-gcc

default: integration

publish-crate:
	cargo publish -p ckb-std

publish: publish-crate

clean:
	cargo clean && make -C test clean

test-shared-lib:
	make -C test/shared-lib all-via-docker

integration: check

test:
	make -C test test

check:
	cargo check --target ${TARGET} --examples

.PHONY: test check
