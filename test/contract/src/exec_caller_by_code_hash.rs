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
    // 0x9315a5f423f183765cbce1edcd075c027991ff706115629b0d4e61e00b3a608f
    let code_hash = [
        0x93, 0x15, 0xa5, 0xf4, 0x23, 0xf1, 0x83, 0x76, 0x5c, 0xbc, 0xe1, 0xed, 0xcd, 0x07, 0x5c,
        0x02, 0x79, 0x91, 0xff, 0x70, 0x61, 0x15, 0x62, 0x9b, 0x0d, 0x4e, 0x61, 0xe0, 0x0b, 0x3a,
        0x60, 0x8f,
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
