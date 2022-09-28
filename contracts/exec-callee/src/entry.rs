// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use core::ffi::CStr;
use core::slice::from_raw_parts;

use crate::error::Error;

pub unsafe fn main(
    argc: core::ffi::c_int,
    argv: *const *const core::ffi::c_char,
) -> Result<(), Error> {
    let args = from_raw_parts(argv, argc as usize);

    let arg1 = CStr::from_ptr(args[0]);
    let arg2 = CStr::from_ptr(args[1]);

    assert_eq!(argc, 2);
    assert_eq!(arg1.to_bytes(), b"Hello World");
    assert_eq!(arg2.to_bytes(), "你好".as_bytes());
    Ok(())
}
