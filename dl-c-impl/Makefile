
TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
AR := $(TARGET)-ar
CFLAGS := -O3 -fno-builtin-printf -fno-builtin-memcmp -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -fdata-sections -ffunction-sections -I ./ckb-c-stdlib -I ./ckb-c-stdlib/libc -I ./ckb-c-stdlib/molecule -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -D__SHARED_LIBRARY__
LDFLAGS := -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

# docker pull nervos/ckb-riscv-gnu-toolchain:gnu-bionic-20191012
BUILDER_DOCKER := nervos/ckb-riscv-gnu-toolchain@sha256:aae8a3f79705f67d505d1f1d5ddc694a4fd537ed1c7e9622420a470d59ba2ec3

all: build/libdl-c-impl.a


build/libdl-c-impl.a: build/lib.o
	$(AR) rcs $@ $^

build/%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f build/*.o build/*.a
