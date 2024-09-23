test: build
	RUST_LOG=debug cargo test -- --nocapture
	make -C simulator build
	make -C simulator run
	make -C simulator testcase

build:
	make -C shared-lib all-via-docker
	cd ../contracts && RUSTFLAGS="-C target-feature=-a" cargo build --target riscv64imac-unknown-none-elf

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
