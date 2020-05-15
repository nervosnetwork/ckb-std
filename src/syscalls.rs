use crate::ckb_constants::*;

#[link(name = "ckb-syscall")]
extern "C" {
    fn syscall(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64;
}

pub fn exit(code: i8) -> ! {
    unsafe { syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}

fn syscall_load(
    buf: &mut [u8],
    len: usize,
    offset: usize,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    syscall_num: u64,
) -> Result<usize, SysError> {
    let mut actual_data_len = len;
    let len_ptr: *mut usize = &mut actual_data_len;
    let buf_ptr: *mut u8 = buf.as_mut_ptr();
    let ret = unsafe {
        syscall(
            buf_ptr as u64,
            len_ptr as u64,
            offset as u64,
            a3,
            a4,
            a5,
            a6,
            syscall_num,
        )
    };
    SysError::build_syscall_result(ret, len, actual_data_len)
}

pub fn load_tx_hash(buf: &mut [u8], len: usize, offset: usize) -> Result<usize, SysError> {
    syscall_load(buf, len, offset, 0, 0, 0, 0, SYS_LOAD_TX_HASH)
}

pub fn load_script_hash(buf: &mut [u8], len: usize, offset: usize) -> Result<usize, SysError> {
    syscall_load(buf, len, offset, 0, 0, 0, 0, SYS_LOAD_SCRIPT_HASH)
}

pub fn load_cell(
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_WITNESS,
    )
}

pub fn load_transaction(buf: &mut [u8], len: usize, offset: usize) -> Result<usize, SysError> {
    syscall_load(buf, len, offset, 0, 0, 0, 0, SYS_LOAD_TRANSACTION)
}

pub fn load_cell_by_field(
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    memory_size: usize,
    content_offset: usize,
    content_size: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
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
    buf: &mut [u8],
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf,
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL_DATA,
    )
}

pub fn load_script(buf: &mut [u8], len: usize, offset: usize) -> Result<usize, SysError> {
    syscall_load(buf, len, offset, 0, 0, 0, 0, SYS_LOAD_SCRIPT)
}

pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    unsafe {
        syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_DEBUG);
    }
}
