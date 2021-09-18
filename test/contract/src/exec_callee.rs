#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{
    cstr_core::{cstr, CStr},
    default_alloc, entry,
};

use core::slice::from_raw_parts;

#[no_mangle]
pub fn main(argc: u64, argv: *const *const u8) -> i8 {
    let args = unsafe { from_raw_parts(argv, argc as usize) };
    let arg1 = unsafe { CStr::from_ptr(args[0]) };
    let arg2 = unsafe { CStr::from_ptr(args[1]) };
    assert_eq!(argc, 2);
    assert_eq!(arg1, cstr!("Hello World"));
    assert_eq!(arg2, cstr!("你好"));
    0
}

entry!(main);
default_alloc!();
