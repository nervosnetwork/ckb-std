#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_contract_std::{debug, setup};
use alloc::vec;

#[no_mangle]
pub fn main() -> i8 {
    let v = vec![0u8;42];
    debug!("{:?}", v.len());
    0
}

setup!(main);
