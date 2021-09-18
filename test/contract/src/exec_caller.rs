#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{ckb_constants::*, cstr_core::cstr, default_alloc, entry, syscalls};

#[no_mangle]
pub fn main() -> i8 {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    let ret = syscalls::exec(0, Source::CellDep, 0, 0, &[arg1, arg2][..]);
    panic!("exec failed: {}", ret);
}

entry!(main);
default_alloc!();
