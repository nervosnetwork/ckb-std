// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html

use crate::error::Error;
extern crate alloc;

pub fn main() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    ckb_std::debug!("argv: {:?}", argv);
    assert_eq!(argv.len(), 2);
    assert_eq!(argv[0].to_bytes(), b"Hello World");
    assert_eq!(argv[1].to_bytes(), "你好".as_bytes());
    Ok(())
}
