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
    // 0x27ce272a3b51f2cd8bc684a9e16ae0ee8021710dcf370a14c592da9cb910efb7

    let code_hash = [
        0x27, 0xce, 0x27, 0x2a, 0x3b, 0x51, 0xf2, 0xcd, 0x8b, 0xc6, 0x84, 0xa9, 0xe1, 0x6a, 0xe0,
        0xee, 0x80, 0x21, 0x71, 0x0d, 0xcf, 0x37, 0x0a, 0x14, 0xc5, 0x92, 0xda, 0x9c, 0xb9, 0x10,
        0xef, 0xb7,
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
