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
    // 0xa6970d35a0f17190f1fd1885ccb2fb01624632c3492546973fd41e243e5180f4

    let code_hash = [
        0xa6, 0x97, 0x0d, 0x35, 0xa0, 0xf1, 0x71, 0x90, 0xf1, 0xfd, 0x18, 0x85, 0xcc, 0xb2, 0xfb,
        0x01, 0x62, 0x46, 0x32, 0xc3, 0x49, 0x25, 0x46, 0x97, 0x3f, 0xd4, 0x1e, 0x24, 0x3e, 0x51,
        0x80, 0xf4,
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
