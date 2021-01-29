
TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
AR := $(TARGET)-ar
CFLAGS := -O3 -fno-builtin-printf -fno-builtin-memcmp -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -fdata-sections -ffunction-sections -I ./ckb-c-stdlib -I ./ckb-c-stdlib/libc -I ./ckb-c-stdlib/molecule -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -D__SHARED_LIBRARY__
LDFLAGS := -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

BUILDER_DOCKER := jjy0/ckb-capsule-recipe-rust:2020-9-28

all: build/libdl-c-impl.a

all-via-docker:
	docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && make all"

build/libdl-c-impl.a: build/lib.o
	$(AR) rcs $@ $^

build/%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f build/*.o build/*.a
