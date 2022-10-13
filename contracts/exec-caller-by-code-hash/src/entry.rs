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
    // 0xa20f0724467451b1a604a6ecb1418c156b6ed069d43964b676777630ac9949a8
    let code_hash = [
        0xa2, 0x0f, 0x07, 0x24, 0x46, 0x74, 0x51, 0xb1, 0xa6, 0x04, 0xa6, 0xec, 0xb1, 0x41, 0x8c,
        0x15, 0x6b, 0x6e, 0xd0, 0x69, 0xd4, 0x39, 0x64, 0xb6, 0x76, 0x77, 0x76, 0x30, 0xac, 0x99,
        0x49, 0xa8,
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
