
build:
	cargo build

run:
	CKB_RUNNING_SETUP=data/it_works-setup.json CKB_TX_FILE=data/it_works-mock-tx.json target/debug/contract
	CKB_RUNNING_SETUP=data/test_exec_by_code_hash-setup.json CKB_TX_FILE=data/test_exec_by_code_hash-mock-tx.json target/debug/exec-caller-by-code-hash

build-simulator:
	cd ../../contracts/exec-callee/exec-callee-dbg/ && cargo build

testcase: build-simulator
	cargo test