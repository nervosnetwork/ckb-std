test: build build-examples
	RUST_LOG=debug cargo test -- --nocapture
	make -C simulator build
	make -C simulator run
	make -C simulator testcase

build:
	make -C shared-lib all-via-docker
	cd ../contracts && RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf

build-examples:
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --features build-with-clang --example always_failure --target riscv64imac-unknown-none-elf
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --features build-with-clang --example always_success --target riscv64imac-unknown-none-elf
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --features build-with-clang --example atomic --target riscv64imac-unknown-none-elf
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --features build-with-clang --example type_id --target riscv64imac-unknown-none-elf --features "type-id"

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
