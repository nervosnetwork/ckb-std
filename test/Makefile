test: clean build
	RUST_LOG=debug cargo test -- --nocapture
	make -C simulator build
	make -C simulator run

build:
	make -C shared-lib all-via-docker
	cd .. && capsule build

clean:
	rm -rf ../build
	cargo clean
	rm -rf shared-lib/shared-lib.so
	rm -rf shared-lib/shared-lib.so.debug
