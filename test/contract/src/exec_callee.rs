#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{cstr_core::CStr, debug, default_alloc, entry};

use core::slice::from_raw_parts;

#[no_mangle]
pub fn main(argc: u64, argv: *const *const u8) -> i8 {
    debug!("argc: {}", argc);
    let args = unsafe { from_raw_parts(argv, argc as usize) };
    for &arg in args {
        debug!(
            "arg: {:?}",
            unsafe { CStr::from_ptr(arg) }.to_str().unwrap()
        );
    }
    0
}

entry!(main);
default_alloc!();
