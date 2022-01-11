#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::vec;
use ckb_std::{debug, default_alloc, entry};
use core::arch::asm;

#[no_mangle]
pub fn main() -> i8 {
    let v = vec![0u8; 42];
    debug!("{:?}", v.len());
    0
}

entry!(main);
default_alloc!();
