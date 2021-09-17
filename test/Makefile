test: clean build patch
	RUST_LOG=debug cargo test -- --nocapture

build:
	make -C shared-lib shared-lib.so
	cd contract && cargo build

C := contract/target/riscv64imac-unknown-none-elf/debug/contract
EXEC_CALLER := contract/target/riscv64imac-unknown-none-elf/debug/exec-caller
EXEC_CALLEE := contract/target/riscv64imac-unknown-none-elf/debug/exec-callee
patch:
	ckb-binary-patcher -i $C -o $C
	ckb-binary-patcher -i ${EXEC_CALLER} -o ${EXEC_CALLER}
	ckb-binary-patcher -i ${EXEC_CALLEE} -o ${EXEC_CALLEE}

clean:
	cd contract && cargo clean
