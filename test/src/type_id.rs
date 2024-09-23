use std::fs::File;
use std::io::Read;

use ckb_hash::new_blake2b;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;
const MAX_CYCLES: u64 = 1000_0000;

fn build_bins() -> (Bytes, Bytes) {
    let always_success_bin = {
        let mut buf = Vec::new();
        File::open("../target/riscv64imac-unknown-none-elf/debug/examples/main")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let type_id_bin = {
        let mut buf = Vec::new();
        File::open("../target/riscv64imac-unknown-none-elf/debug/examples/type_id")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    (always_success_bin, type_id_bin)
}

fn type_id_mint(wrong_type_id: bool) {
    let mut context = Context::default();
    let (always_success_bin, type_id_bin) = build_bins();
    let always_success_out_point = context.deploy_cell(always_success_bin);
    let type_id_out_point = context.deploy_cell(type_id_bin);
    let type_script_dep = CellDep::new_builder()
        .out_point(type_id_out_point.clone())
        .build();

    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let index: usize = 0;
    let mut type_id = vec![0u8; 32];
    let mut blake2b = new_blake2b();
    blake2b.update(input.as_slice());
    blake2b.update(&index.to_be_bytes());
    blake2b.finalize(&mut type_id);
    if wrong_type_id {
        type_id[0] ^= 1;
    }
    let type_script = context
        .build_script(&type_id_out_point, type_id.into())
        .expect("script");

    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(Some(type_script.clone()).pack())
        .build()];

    let mut outputs_data: Vec<Bytes> = Vec::new();
    outputs_data.push(vec![42u8; 1000].into());

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let result = context.verify_tx(&tx, MAX_CYCLES);
    if wrong_type_id {
        result.expect_err("should verify failed");
    } else {
        result.expect("should verify success");
    }
}

fn type_id_tx(wrong_type_id: bool) {
    let mut context = Context::default();
    let (always_success_bin, type_id_bin) = build_bins();
    let always_success_out_point = context.deploy_cell(always_success_bin);
    let type_id_out_point = context.deploy_cell(type_id_bin);
    let type_script_dep = CellDep::new_builder()
        .out_point(type_id_out_point.clone())
        .build();

    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();
    let type_id = vec![1u8; 32];
    let type_script = context
        .build_script(&type_id_out_point, type_id.into())
        .expect("script");

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let type_id2 = if wrong_type_id {
        vec![2u8; 32]
    } else {
        vec![1u8; 32]
    };
    let type_script2 = context
        .build_script(&type_id_out_point, type_id2.into())
        .expect("script");

    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(Some(type_script2.clone()).pack())
        .build()];

    let mut outputs_data: Vec<Bytes> = Vec::new();
    outputs_data.push(vec![42u8; 1000].into());

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let result = context.verify_tx(&tx, MAX_CYCLES);
    if wrong_type_id {
        result.expect_err("should verify failed");
    } else {
        result.expect("should verify success");
    }
}
#[test]
fn test_type_id_mint() {
    type_id_mint(false);
}

#[test]
fn test_type_id_mint_failed() {
    type_id_mint(true);
}

#[test]
fn test_type_id_tx() {
    type_id_tx(false);
}

#[test]
fn test_type_id_tx_failed() {
    type_id_tx(true);
}
