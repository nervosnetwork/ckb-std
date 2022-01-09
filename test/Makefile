test: clean build
	RUST_LOG=debug cargo test -- --nocapture

build:
	make -C shared-lib all-via-docker
	@./build_test_contracts.sh

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
