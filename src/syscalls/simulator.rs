use crate::{ckb_constants::*, error::SysError};
use ckb_x64_simulator as sim;
use core::ffi::c_void;

pub fn exit(code: i8) -> ! {
    sim::ckb_exit(code);
    loop {}
}

pub fn load_tx_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_tx_hash(buf.as_mut_ptr() as *mut c_void, len_ptr, offset as u64);
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_script_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_script_hash(buf.as_mut_ptr() as *mut c_void, len_ptr, offset as u64);
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_cell(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_cell(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_input(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_input(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_header(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_header(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_witness(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_witness(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_transaction(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_transaction(buf.as_mut_ptr() as *mut c_void, len_ptr, offset as u64);
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_cell_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_cell_by_field(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_header_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_header_by_field(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_input_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_input_by_field(
        buf.as_mut_ptr() as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn load_cell_data(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    load_cell_data_raw(buf.as_mut_ptr(), buf.len(), offset, index, source)
}

pub fn load_script(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    let mut actual_data_len = buf.len() as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_script(buf.as_mut_ptr() as *mut c_void, len_ptr, offset as u64);
    SysError::build_syscall_result(ret as i64 as u64, buf.len(), actual_data_len as usize)
}

pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    sim::ckb_debug(c_str.as_ptr() as *const i8);
}

pub fn load_cell_data_raw(
    buf_ptr: *mut u8,
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut actual_data_len = len as u64;
    let len_ptr: *mut u64 = &mut actual_data_len;
    let ret = sim::ckb_load_header(
        buf_ptr as *mut c_void,
        len_ptr,
        offset as u64,
        index as u64,
        source as u64,
    );
    SysError::build_syscall_result(ret as i64 as u64, len, actual_data_len as usize)
}

pub fn load_cell_code(
    _buf_ptr: *mut u8,
    _len: usize,
    _content_offset: usize,
    _content_size: usize,
    _index: usize,
    _source: Source,
) -> Result<usize, SysError> {
    panic!("This is not supported in the simulator!");
}
