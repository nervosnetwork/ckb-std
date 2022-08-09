// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{ckb_types::core::ScriptHashType, cstr_core::cstr, high_level};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    // $ ckb-cli util blake2b --binary-path build/debug/exec-callee
    // 0x9b5f7e792a98a8471681f5c4b3d80c671fae7855393536f811daa2e63ca85704

    let code_hash = [
        0x9b, 0x5f, 0x7e, 0x79, 0x2a, 0x98, 0xa8, 0x47, 0x16, 0x81, 0xf5, 0xc4, 0xb3, 0xd8, 0x0c,
        0x67, 0x1f, 0xae, 0x78, 0x55, 0x39, 0x35, 0x36, 0xf8, 0x11, 0xda, 0xa2, 0xe6, 0x3c, 0xa8,
        0x57, 0x04,
    ];
    ckb_std::debug!("code_hash: {:?}", code_hash);
    let ret = high_level::exec_cell(
        &code_hash[..],
        ScriptHashType::Data1,
        0,
        0,
        &[arg1, arg2][..],
    )
    .unwrap();
    panic!("exec failed: {}", ret);
}
