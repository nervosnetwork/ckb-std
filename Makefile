TARGET := riscv64imac-unknown-none-elf
CC := riscv64-unknown-elf-gcc

default: integration

publish-crate:
	cross publish -p ckb-standalone-types && cross publish -p ckb-std

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

