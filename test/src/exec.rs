use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;
use std::fs::File;
use std::io::Read;

const MAX_CYCLES: u64 = 1000_0000;

#[test]
fn test_exec() {
    let mut context = Context::default();
    let caller_bin = {
        let mut buf = Vec::new();
        File::open("contract/target/riscv64imac-unknown-none-elf/debug/exec-caller")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = {
        let mut buf = Vec::new();
        File::open("contract/target/riscv64imac-unknown-none-elf/debug/exec-callee")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let callee_out_point = context.deploy_cell(callee_bin);

    let caller_lock_script_dep = CellDep::new_builder()
        .out_point(caller_out_point.clone())
        .build();
    let callee_lock_script_dep = CellDep::new_builder().out_point(callee_out_point).build();

    let caller_lock_script = context
        .build_script(&caller_out_point, Bytes::new())
        .unwrap();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(caller_lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(ScriptBuilder::default().build())
        .build()];
    let outputs_data = vec![Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(callee_lock_script_dep)
        .cell_dep(caller_lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}
