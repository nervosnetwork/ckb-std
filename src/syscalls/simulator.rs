use crate::{ckb_constants::*, error::SysError};
use ckb_types::core::ScriptHashType;
use ckb_x64_simulator as sim;
use core::convert::Infallible;
use core::ffi::{c_void, CStr};

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
    let ret = sim::ckb_load_cell_data(
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
    panic!("This is not supported in the native-simulator!");
}

pub fn vm_version() -> Result<u64, SysError> {
    let ret = sim::ckb_vm_version();
    if ret == 1 {
        Ok(1)
    } else {
        Err(SysError::Unknown(ret as i64 as u64))
    }
}

pub fn current_cycles() -> u64 {
    sim::ckb_current_cycles()
}

pub fn exec(
    _index: usize,
    _source: Source,
    _place: usize,
    _bounds: usize,
    // argc: i32,
    _argv: &[&CStr],
) -> u64 {
    panic!("please use exec_cell instead");
}

pub fn exec_cell(
    code_hash: &[u8],
    hash_type: ScriptHashType,
    argv: &[&CStr],
) -> Result<Infallible, SysError> {
    let argc = argv.len();
    let mut argv_vec: alloc::vec::Vec<*const u8> =
        argv.iter().map(|e| e.as_ptr() as *const u8).collect();
    argv_vec.push(core::ptr::null());
    let ret = sim::ckb_exec_cell(
        code_hash.as_ptr(),
        hash_type as u8,
        0,
        0,
        argc as i32,
        argv_vec.as_ptr(),
    );
    Err(SysError::Unknown(ret as u64))
}

#[repr(C)]
pub struct SpawnArgs {
    /// argc contains the number of arguments passed to the program.
    pub argc: u64,
    /// argv is a one-dimensional array of strings.
    pub argv: *const *const i8,
    /// a pointer used to save the process_id of the child process.
    pub process_id: *mut u64,
    /// an array representing the file descriptors passed to the child process. It must end with zero.
    pub inherited_fds: *const u64,
}

pub fn spawn(
    _index: usize,
    _source: Source,
    _place: usize,
    _bounds: usize,
    _spgs: &mut SpawnArgs,
) -> Result<u64, SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn wait(_pid: u64) -> Result<i8, SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn process_id() -> u64 {
    panic!("This is not supported in the native-simulator!");
}

pub fn pipe() -> Result<(u64, u64), SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn read(_fd: u64, _buffer: &mut [u8]) -> Result<usize, SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn write(_fd: u64, _buffer: &[u8]) -> Result<usize, SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn inherited_fds(_fds: &mut [u64]) -> u64 {
    panic!("This is not supported in the native-simulator!");
}

pub fn close(_fd: u64) -> Result<(), SysError> {
    panic!("This is not supported in the native-simulator!");
}

pub fn load_block_extension(
    _buf: &mut [u8],
    _offset: usize,
    _index: usize,
    _source: Source,
) -> Result<usize, SysError> {
    panic!("This is not supported in the native-simulator!");
}
