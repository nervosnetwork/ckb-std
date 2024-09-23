test: build build-examples
	RUST_LOG=debug cargo test -- --nocapture
	make -C simulator build
	make -C simulator run
	make -C simulator testcase

build:
	make -C shared-lib all-via-docker
	cd ../contracts && RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf

build-examples:
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --example type_id --target riscv64imac-unknown-none-elf --features "type-id" 
	cd ../examples && RUSTFLAGS="-C target-feature=-a" cargo build --example main --target riscv64imac-unknown-none-elf

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
