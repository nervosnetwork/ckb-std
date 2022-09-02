// Import from `core` instead of from `std` since we are in no-std mode
use core::ffi::CStr;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{ckb_types::core::ScriptHashType, high_level};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let arg1 = CStr::from_bytes_with_nul(b"Hello World\0").unwrap();
    let arg2 = CStr::from_bytes_with_nul("你好\0".as_bytes()).unwrap();
    // $ ckb-cli util blake2b --binary-path build/debug/exec-callee
    // 0xad1a71e71353b2439c0e4d7d634f0cdb2c56faad755500bed07cbe76a71e6000
    let code_hash = [
        0xad, 0x1a, 0x71, 0xe7, 0x13, 0x53, 0xb2, 0x43, 0x9c, 0x0e, 0x4d, 0x7d, 0x63, 0x4f, 0x0c,
        0xdb, 0x2c, 0x56, 0xfa, 0xad, 0x75, 0x55, 0x00, 0xbe, 0xd0, 0x7c, 0xbe, 0x76, 0xa7, 0x1e,
        0x60, 0x00,
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
