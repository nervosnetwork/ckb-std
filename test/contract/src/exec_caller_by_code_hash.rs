#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{ckb_types::core::ScriptHashType, cstr_core::cstr, default_alloc, entry, high_level};

#[no_mangle]
pub fn main() -> i8 {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    // $ ckb-cli util blake2b --binary-path target/riscv64imac-unknown-none-elf/debug/exec-callee
    // 0x6c704a9d5084cd52ff3a0be1f785b5f0fb9663092f8083bef7fc3f547758cb5a
    let code_hash = [
        0x6c, 0x70, 0x4a, 0x9d, 0x50, 0x84, 0xcd, 0x52, 0xff, 0x3a, 0x0b, 0xe1, 0xf7, 0x85, 0xb5,
        0xf0, 0xfb, 0x96, 0x63, 0x09, 0x2f, 0x80, 0x83, 0xbe, 0xf7, 0xfc, 0x3f, 0x54, 0x77, 0x58,
        0xcb, 0x5a,
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

entry!(main);
default_alloc!();
