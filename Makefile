TARGET := riscv64imac-unknown-none-elf
CC := riscv64-unknown-elf-gcc

default: integration

publish-crate:
	cargo publish --target ${TARGET} -p ckb-std

publish-crate-dryrun:
	cargo publish --dry-run --target ${TARGET} -p ckb-std --allow-dirty

publish: publish-crate

clean:
	cargo clean && make -C test clean

test-shared-lib:
	make -C test/shared-lib all-via-docker

integration: check

test: publish-crate-dryrun
	make -C test test

check:
	cargo check --target ${TARGET} --examples

.PHONY: test check
