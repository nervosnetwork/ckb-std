#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::vec;
use ckb_contract_std::{debug, setup, syscalls, ckb_constants::*};
use core::mem::size_of;

fn test_basic(){
    let v = vec![0u8;42];
    debug!("{:?}", v.len());
}

fn test_load_cell_field() {
    let raw = syscalls::load_cell_by_field(size_of::<u64>(), 0, 0, Source::GroupInput, CellField::Capacity).unwrap();
    let mut buf = [0u8;size_of::<u64>()];
    buf.clone_from_slice(&raw);
    let capacity = u64::from_le_bytes(buf);
    debug!("input capacity {}", capacity);
}

fn test_load_tx_hash() {
    let tx_hash = syscalls::load_tx_hash(32, 0).unwrap();
    debug!("tx hash {:?}", tx_hash);
}

#[no_mangle]
pub fn main() -> i8 {
    test_basic();
    test_load_cell_field();
    test_load_tx_hash();
    0
}

setup!(main);
