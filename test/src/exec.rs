use super::util::{blake2b_256, dump_mock_tx};
use ckb_testtool::{
    ckb_traits::CellDataProvider,
    ckb_types::{
        bytes::Bytes,
        core::{ScriptHashType, TransactionBuilder},
        packed::*,
        prelude::*,
    },
    context::Context,
};
use ckb_x64_simulator::RunningSetup;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const MAX_CYCLES: u64 = 1000_0000;

#[test]
fn test_exec_by_index() {
    let mut context = Context::default();
    let caller_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-caller")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-callee")
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

#[test]
fn test_exec_by_code_hash() {
    let mut context = Context::default();
    let caller_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-caller-by-code-hash")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-callee")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let callee_out_point = context.deploy_cell(callee_bin.clone());
    let callee_code_hash = context
        .get_cell_data_hash(&callee_out_point)
        .unwrap()
        .as_bytes();

    let caller_lock_script_dep = CellDep::new_builder()
        .out_point(caller_out_point.clone())
        .build();
    let callee_lock_script_dep = CellDep::new_builder().out_point(callee_out_point).build();

    let caller_lock_script = context
        .build_script(&caller_out_point, callee_code_hash)
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

    let test_case_name = "test_exec_by_code_hash";
    let mut native_binaries = HashMap::default();
    let key_string = {
        let mut buffer = Vec::with_capacity(32 + 1 + 4 + 4);
        // See entry_exec_caller_by_code_hash.rs for the arguments to choose
        buffer.extend_from_slice(&blake2b_256(callee_bin.as_ref())[..]);
        buffer.push(ScriptHashType::Data1 as u8);
        buffer.extend_from_slice(&0u32.to_be_bytes()[..]);
        buffer.extend_from_slice(&0u32.to_be_bytes()[..]);
        format!("0x{}", faster_hex::hex_string(&buffer))
    };
    println!("binary key: {}", key_string);
    native_binaries.insert(key_string, "target/debug/exec-callee".to_string());
    let setup = RunningSetup {
        is_lock_script: true,
        is_output: false,
        script_index: 0,
        vm_version: 1,
        native_binaries,
    };
    dump_mock_tx(test_case_name, &tx, &context, &setup);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}

#[test]
fn test_exec_args() {
    let mut context = Context::default();
    let caller_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-caller-args")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let caller_out_point = context.deploy_cell(caller_bin);
    let callee_bin = {
        let mut buf = Vec::new();
        File::open("../build/debug/exec-callee-args")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let callee_out_point = context.deploy_cell(callee_bin.clone());
    let callee_code_hash = context
        .get_cell_data_hash(&callee_out_point)
        .unwrap()
        .as_bytes();

    let caller_lock_script_dep = CellDep::new_builder()
        .out_point(caller_out_point.clone())
        .build();
    let callee_lock_script_dep = CellDep::new_builder().out_point(callee_out_point).build();

    let caller_lock_script = context
        .build_script(&caller_out_point, callee_code_hash)
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

    let test_case_name = "test_exec_args";
    let mut native_binaries = HashMap::default();
    let key_string = {
        let mut buffer = Vec::with_capacity(32 + 1 + 4 + 4);
        // See entry_exec_caller_by_code_hash.rs for the arguments to choose
        buffer.extend_from_slice(&blake2b_256(callee_bin.as_ref())[..]);
        buffer.push(ScriptHashType::Data1 as u8);
        buffer.extend_from_slice(&0u32.to_be_bytes()[..]);
        buffer.extend_from_slice(&0u32.to_be_bytes()[..]);
        format!("0x{}", faster_hex::hex_string(&buffer))
    };
    println!("binary key: {}", key_string);
    native_binaries.insert(key_string, "target/debug/exec-callee-args".to_string());
    let setup = RunningSetup {
        is_lock_script: true,
        is_output: false,
        script_index: 0,
        vm_version: 1,
        native_binaries,
    };
    dump_mock_tx(test_case_name, &tx, &context, &setup);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consumed cycles {}", cycles);
}
