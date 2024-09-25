#![no_main]
#![no_std]

// Copied from https://github.com/rust-lang/rust/blob/master/library/core/tests/atomic.rs.

use ckb_std::{default_alloc, entry};
use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::*;

entry!(main);
default_alloc!();

fn bool_compare_exchange() {
    let a = AtomicBool::new(false);
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Ok(false));
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Err(true));

    a.store(false, SeqCst);
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Ok(false));
}

fn bool_and() {
    let a = AtomicBool::new(true);
    assert_eq!(a.fetch_and(false, SeqCst), true);
    assert_eq!(a.load(SeqCst), false);
}

fn bool_nand() {
    let a = AtomicBool::new(false);
    assert_eq!(a.fetch_nand(false, SeqCst), false);
    assert_eq!(a.load(SeqCst), true);
    assert_eq!(a.fetch_nand(false, SeqCst), true);
    assert_eq!(a.load(SeqCst), true);
    assert_eq!(a.fetch_nand(true, SeqCst), true);
    assert_eq!(a.load(SeqCst), false);
    assert_eq!(a.fetch_nand(true, SeqCst), false);
    assert_eq!(a.load(SeqCst), true);
}

fn uint_and() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_and(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 & 0x137f);
}

fn uint_nand() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_nand(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), !(0xf731 & 0x137f));
}

fn uint_or() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_or(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 | 0x137f);
}

fn uint_xor() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_xor(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 ^ 0x137f);
}

fn uint_min() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_min(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0x137f);
    assert_eq!(x.fetch_min(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0x137f);
}

fn uint_max() {
    let x = AtomicUsize::new(0x137f);
    assert_eq!(x.fetch_max(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0xf731);
    assert_eq!(x.fetch_max(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731);
}

fn main() -> i8 {
    bool_compare_exchange();
    bool_and();
    bool_nand();
    uint_and();
    uint_nand();
    uint_or();
    uint_xor();
    uint_min();
    uint_max();
    return 0;
}
