test: build build-examples
	RUST_LOG=debug cargo test -- --nocapture
	make -C simulator build
	make -C simulator run
	make -C simulator testcase

build:
	make -C shared-lib all-via-docker
	cd ../contracts && RUSTFLAGS="-C target-feature=-a" cargo build --release --target riscv64imac-unknown-none-elf

build-examples:
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --release --target riscv64imac-unknown-none-elf --features build-with-clang --example always_failure
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --release --target riscv64imac-unknown-none-elf --features build-with-clang --example always_success
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --release --target riscv64imac-unknown-none-elf --features build-with-clang --example atomic
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --release --target riscv64imac-unknown-none-elf --features build-with-clang,type-id --example type_id

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
