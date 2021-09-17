#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]


use alloc::vec;
use alloc::vec::Vec;
use blake2b_ref::{Blake2b, Blake2bBuilder};
use ckb_std::{
    cstr_core::{cstr, CString},
    ckb_constants::*, ckb_types::prelude::*, debug, default_alloc, dynamic_loading, entry,
    error::SysError, high_level, syscalls
};

use core::mem::{size_of, size_of_val};

fn new_blake2b() -> Blake2b {
    const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
    Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build()
}



#[no_mangle]
pub fn main() -> i8 {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    let ret = syscalls::exec(0, Source::CellDep, 0, 0, &[arg1, arg2][..]);
    panic!("exec failed: {}", ret);
}

entry!(main);
default_alloc!();
