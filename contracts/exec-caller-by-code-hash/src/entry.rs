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
    // 0x17d2e30fc9c689cd329d66d7bd0d7922a4a922b547d14ca9e7c0f54cb19d8f29

    let code_hash = [
        0x17, 0xd2, 0xe3, 0x0f, 0xc9, 0xc6, 0x89, 0xcd, 0x32, 0x9d, 0x66, 0xd7, 0xbd, 0x0d, 0x79,
        0x22, 0xa4, 0xa9, 0x22, 0xb5, 0x47, 0xd1, 0x4c, 0xa9, 0xe7, 0xc0, 0xf5, 0x4c, 0xb1, 0x9d,
        0x8f, 0x29,
    ];
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
