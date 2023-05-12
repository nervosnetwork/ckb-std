TARGET := riscv64-unknown-linux-gnu
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
OBJCOPY := $(TARGET)-objcopy
CFLAGS := -fPIC -O3 -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -I deps/ckb-c-stdlib -I deps/ckb-c-stdlib/libc -I deps -I deps/molecule -I c -I build -I deps/secp256k1/src -I deps/secp256k1 -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -g
LDFLAGS := -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

# docker pull nervos/ckb-riscv-gnu-toolchain:gnu-focal-20230214
BUILDER_DOCKER := nervos/ckb-riscv-gnu-toolchain@sha256:7bc4e566f293b6c3d9740cf04fc5861b2f0acc268ca3a025fcf9446f1ab5f27d

all-via-docker:
	docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && make shared-lib.so"

shared-lib.so: shared-lib.c
	$(CC) $(CFLAGS) $(LDFLAGS) -shared -o $@ $<
	$(OBJCOPY) --only-keep-debug $@ $@.debug
	$(OBJCOPY) --strip-debug --strip-all $@
