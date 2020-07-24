test: clean build patch
	cargo test -- --nocapture

build:
	cd contract && cargo build

C := contract/target/riscv64imac-unknown-none-elf/debug/contract
patch:
	ckb-binary-patcher -i $C -o $C

clean:
	cd contract && cargo clean
