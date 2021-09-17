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
    // 0x2f47fd0d54a0d02f5218c5678e6c2cf4358196907e7a033234fdc429843df7af
    let code_hash = [
        0x2f, 0x47, 0xfd, 0x0d, 0x54, 0xa0, 0xd0, 0x2f, 0x52, 0x18, 0xc5, 0x67, 0x8e, 0x6c, 0x2c,
        0xf4, 0x35, 0x81, 0x96, 0x90, 0x7e, 0x7a, 0x03, 0x32, 0x34, 0xfd, 0xc4, 0x29, 0x84, 0x3d,
        0xf7, 0xaf,
    ];
    let ret = high_level::exec_cell(
        &code_hash[..],
        ScriptHashType::Data,
        0,
        0,
        &[arg1, arg2][..],
    )
    .unwrap();
    panic!("exec failed: {}", ret);
}

entry!(main);
default_alloc!();
