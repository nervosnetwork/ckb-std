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
    if ret == 1 || ret == 2 {
        Ok(ret as u64)
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

pub use sim::SpawnArgs;

pub fn spawn_cell(
    code_hash: &[u8],
    hash_type: ScriptHashType,
    argv: &[&CStr],
    inherited_fds: &[u64],
) -> Result<u64, SysError> {
    let argc = argv.len();
    let mut argv_vec: alloc::vec::Vec<*const u8> =
        argv.iter().map(|e| e.as_ptr() as *const u8).collect();
    argv_vec.push(core::ptr::null());

    let mut pid = 0u64;
    let ret = sim::ckb_spawn_cell(
        code_hash.as_ptr(),
        hash_type as u8,
        0,
        0,
        argc as i32,
        argv_vec.as_ptr(),
        inherited_fds.as_ptr(),
        &mut pid,
    );
    match ret {
        0 => Ok(pid),
        1 => Err(SysError::IndexOutOfBound),
        2 => Err(SysError::ItemMissing),
        3 => Err(SysError::Encoding),
        6 => Err(SysError::InvalidFd),
        8 => Err(SysError::MaxVmsSpawned),
        x => Err(SysError::Unknown(x as u64)),
    }
}

pub fn spawn(
    _index: usize,
    _source: Source,
    _place: usize,
    _bounds: usize,
    _spgs: &mut SpawnArgs,
) -> Result<u64, SysError> {
    panic!("please use exec_cell instead");
}

pub fn wait(pid: u64) -> Result<i8, SysError> {
    let mut code: i8 = 0;
    let ret = sim::ckb_wait(pid, (&mut code) as *mut i8) as u64;
    match ret {
        0 => Ok(code as i8),
        5 => Err(SysError::WaitFailure),
        x => Err(SysError::Unknown(x)),
    }
}

pub fn process_id() -> u64 {
    sim::ckb_process_id()
}

pub fn pipe() -> Result<(u64, u64), SysError> {
    let mut fds: [u64; 2] = [0, 0];
    let ret = sim::ckb_pipe(&mut fds as *mut u64) as u64;
    match ret {
        0 => Ok((fds[0], fds[1])),
        9 => Err(SysError::MaxFdsCreated),
        x => Err(SysError::Unknown(x)),
    }
}

pub fn read(fd: u64, buffer: &mut [u8]) -> Result<usize, SysError> {
    let mut len = buffer.len();
    let ret = sim::ckb_read(
        fd,
        buffer.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize,
    ) as u64;
    match ret {
        0 => Ok(len),
        1 => Err(SysError::IndexOutOfBound),
        6 => Err(SysError::InvalidFd),
        7 => Err(SysError::OtherEndClosed),
        x => Err(SysError::Unknown(x)),
    }
}

pub fn write(fd: u64, buffer: &[u8]) -> Result<usize, SysError> {
    let mut l = buffer.len();
    let ret = sim::ckb_write(fd, buffer.as_ptr() as *mut c_void, &mut l as *mut usize) as u64;
    match ret {
        0 => Ok(l as usize),
        1 => Err(SysError::IndexOutOfBound),
        6 => Err(SysError::InvalidFd),
        7 => Err(SysError::OtherEndClosed),
        x => Err(SysError::Unknown(x)),
    }
}

pub fn inherited_fds(fds: &mut [u64]) -> u64 {
    let mut l = fds.len();
    sim::ckb_inherited_fds(fds.as_mut_ptr(), &mut l as *mut usize) as u64
}

pub fn close(fd: u64) -> Result<(), SysError> {
    let ret = sim::ckb_close(fd) as u64;
    match ret {
        0 => Ok(()),
        6 => Err(SysError::InvalidFd),
        x => Err(SysError::Unknown(x)),
    }
}

pub fn load_block_extension(
    buffer: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let mut l: u64 = buffer.len() as u64;
    let ret = sim::ckb_load_block_extension(
        buffer.as_mut_ptr() as *mut c_void,
        &mut l as *mut u64,
        offset,
        index,
        source as usize,
    ) as u64;

    match ret {
        0 => Ok(l as usize),
        1 => Err(SysError::IndexOutOfBound),
        6 => Err(SysError::InvalidFd),
        7 => Err(SysError::OtherEndClosed),
        x => Err(SysError::Unknown(x)),
    }
}
