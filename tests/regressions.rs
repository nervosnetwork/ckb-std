#![cfg(feature = "ckb-types")]

use ckb_std::{
    dynamic_loading::{CKBDLContext, Error},
    error::SysError,
    high_level::{decode_hex, encode_hex},
};
use core::ffi::CStr;

#[test]
fn hex_accepts_empty_mixed_case_and_all_byte_values() {
    assert_eq!(decode_hex(c"").unwrap(), []);
    assert_eq!(decode_hex(c"00aAfF19").unwrap(), [0, 0xaa, 0xff, 0x19]);
    let bytes: Vec<u8> = (0..=255).collect();
    assert_eq!(decode_hex(&encode_hex(&bytes)).unwrap(), bytes);
}

#[test]
fn hex_rejects_malformed_input_without_panicking() {
    for bytes in [
        b"\xff\xff\0".as_slice(),
        "€0\0".as_bytes(),
        b"0\0",
        b"0g\0",
        b"+1\0",
        b" 1\0",
    ] {
        let input = CStr::from_bytes_with_nul(bytes).unwrap();
        assert_eq!(decode_hex(input), Err(SysError::Encoding));
    }
}

#[test]
#[allow(deprecated)]
fn legacy_loader_rejects_invalid_ranges_before_syscalls() {
    let mut context = unsafe { CKBDLContext::<[u8; 8192]>::new() };
    for (offset, size) in [
        (8192, 4096),
        (0, 12288),
        (4096, 8192),
        (usize::MAX - 4095, 4096),
    ] {
        assert_eq!(
            context.load_with_offset(&[0; 32], offset, size).err(),
            Some(Error::ContextFailure)
        );
    }
    assert_eq!(
        context.load_with_offset(&[0; 32], 0, 4097).err(),
        Some(Error::InvalidAlign)
    );
}
