#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::vec;
use ckb_std::{ckb_constants::*, debug, default_alloc, entry, syscalls};
use core::mem::size_of;

fn test_basic() {
    let v = vec![0u8; 42];
    debug!("{:?}", v.len());
}

fn test_load_cell_field() {
    let mut buf = [0u8; size_of::<u64>()];
    let len = syscalls::load_cell_by_field(&mut buf, 0, 0, Source::GroupInput, CellField::Capacity)
        .unwrap();
    assert_eq!(len, buf.len());
    let capacity = u64::from_le_bytes(buf);
    debug!("input capacity {}", capacity);
}

fn test_load_tx_hash() {
    let mut tx_hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
    assert_eq!(len, tx_hash.len());
    debug!("tx hash {:?}", tx_hash);
}

fn test_partial_load_tx_hash() {
    let mut tx_hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
    assert_eq!(len, tx_hash.len());
    assert_ne!(tx_hash, [0u8; 32]);

    // partial load ..16
    let mut buf = [0u8; 16];
    let err = syscalls::load_tx_hash(&mut buf, 0).unwrap_err();
    assert_eq!(err, SysError::LengthNotEnough(32));
    assert_eq!(buf[..], tx_hash[..16]);
    // partial load 16..
    let len = syscalls::load_tx_hash(&mut buf, 16).unwrap();
    assert_eq!(len, buf.len());
    assert_eq!(buf[..], tx_hash[16..]);
}

#[no_mangle]
pub fn main() -> i8 {
    test_basic();
    test_load_cell_field();
    test_load_tx_hash();
    test_partial_load_tx_hash();
    0
}

entry!(main);
default_alloc!();
