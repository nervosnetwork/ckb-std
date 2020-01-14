use crate::ckb_constants::*;
use alloc::vec::Vec;

pub fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    syscall_num: u64,
) -> u64 {
    unsafe {
        asm!("mv a0, $0" :: "r"(a0) : "a0":"volatile");
        asm!("mv a1, $0" :: "r"(a1) : "a1":"volatile");
        asm!("mv a2, $0" :: "r"(a2) : "a2":"volatile");
        asm!("mv a3, $0" :: "r"(a3) : "a3":"volatile");
        asm!("mv a4, $0" :: "r"(a4) : "a4":"volatile");
        asm!("mv a5, $0" :: "r"(a5) : "a5":"volatile");
        asm!("mv a6, $0" :: "r"(a6) : "a6" :"volatile");
        asm!("mv a7, $0" :: "r"(syscall_num) : "a7":"volatile");
        asm!("ecall" : "+r"(a0):::"volatile");
    }
    return a0;
}

pub fn exit(code: i8) -> ! {
    syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT);
    loop {}
}

fn syscall_load(
    mut len: usize,
    offset: usize,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    syscall_num: u64,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    let old_len = len;
    buf.resize(len, 0);
    let len_ptr: *mut usize = &mut len;
    let ret = syscall(
        buf.as_ptr() as u64,
        len_ptr as u64,
        offset as u64,
        a3,
        a4,
        a5,
        a6,
        syscall_num,
    );
    // set buf len
    unsafe {
        buf.set_len(*len_ptr);
    }
    buf.shrink_to_fit();
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > old_len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_tx_hash(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    syscall_load(len, offset, 0, 0, 0, 0, SYS_LOAD_TX_HASH)
}

pub fn load_script_hash(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    syscall_load(len, offset, 0, 0, 0, 0, SYS_LOAD_SCRIPT_HASH)
}

pub fn load_cell(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL,
    )
}

pub fn load_input(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_INPUT,
    )
}

pub fn load_header(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_HEADER,
    )
}

pub fn load_witness(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_WITNESS,
    )
}

pub fn load_transaction(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    syscall_load(len, offset, 0, 0, 0, 0, SYS_LOAD_TRANSACTION)
}

pub fn load_cell_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_CELL_BY_FIELD,
    )
}

pub fn load_header_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_HEADER_BY_FIELD,
    )
}

pub fn load_input_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_INPUT_BY_FIELD,
    )
}

pub fn load_cell_code(
    memory_size: usize,
    content_offset: usize,
    content_size: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        memory_size,
        content_offset,
        content_size as u64,
        index as u64,
        source as u64,
        0,
        SYS_LOAD_CELL_DATA_AS_CODE,
    )
}

pub fn load_cell_data(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    syscall_load(
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL_DATA,
    )
}

pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_DEBUG);
}
