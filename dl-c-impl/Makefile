
TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
AR := $(TARGET)-ar
CFLAGS := -O3 -fno-builtin-printf -fno-builtin-memcmp -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -fdata-sections -ffunction-sections -I ./ckb-c-stdlib -I ./ckb-c-stdlib/libc -I ./ckb-c-stdlib/molecule -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -D__SHARED_LIBRARY__
LDFLAGS := -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

BUILDER_DOCKER := jjy0/ckb-capsule-recipe-rust:2020-9-28

CC2 := gcc
LD2 := gcc
AR2 := ar

all: build/libdl-c-impl.a

all-x86: build-x86/libdl-c-impl.a

all-via-docker:
	docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && make all"

build/libdl-c-impl.a: build/lib.o
	$(AR) rcs $@ $^

build-x86/libdl-c-impl.a: build-x86/lib.o
	$(AR2) rcs $@ $^

build/%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

CFLAGS2:=$(filter-out -Werror,$(CFLAGS))
CFLAGS2:=$(filter-out -Wno-nonnull,$(CFLAGS2))
CFLAGS2:=$(filter-out -Wno-nonnull-compare,$(CFLAGS2))
CFLAGS2:=$(filter-out -Wno-unused-function,$(CFLAGS2))
CFLAGS2:=$(filter-out -Wall,$(CFLAGS2))
CFLAGS2:=$(filter-out -nostartfiles,$(CFLAGS2))
CFLAGS2+=-DCKB_STDLIB_NO_SYSCALL_IMPL

build-x86/%.o: %.c
	$(CC2) $(CFLAGS2) -c -o $@ $<

clean:
	rm -f build/*.o build/*.a build-x86/*.o build-x86/*.a

