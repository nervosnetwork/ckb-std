use crate::ckb_constants::*;
use alloc::vec::Vec;

pub fn syscall(
    mut a0: u64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
    _syscall: u64,
) -> u64 {
    unsafe {
        asm!("ecall" : "+r"(a0));
    }
    return a0;
}

pub fn exit(code: i8) {
    syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT);
}

pub fn load_tx_hash(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        0,
        0,
        0,
        0,
        SYS_LOAD_TX_HASH,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_script_hash(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        0,
        0,
        0,
        0,
        SYS_LOAD_SCRIPT_HASH,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_cell(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_input(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_INPUT,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_header(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_HEADER,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_witness(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_WITNESS,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_transaction(len: usize, offset: usize) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        0,
        0,
        0,
        0,
        SYS_LOAD_TRANSACTION,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_cell_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_CELL_BY_FIELD,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_header_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_HEADER_BY_FIELD,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_input_by_field(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_INPUT_BY_FIELD,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_cell_code(
    memory_size: usize,
    content_offset: usize,
    content_size: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(memory_size, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        memory_size as u64,
        content_offset as u64,
        content_size as u64,
        index as u64,
        source as u64,
        0,
        SYS_LOAD_CELL_DATA_AS_CODE,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > memory_size {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn load_cell_data(
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<Vec<u8>, SysError> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(len, 0);
    let ret = syscall(
        buf.as_ptr() as u64,
        len as u64,
        offset as u64,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL_DATA,
    );
    if ret != CKB_SUCCESS {
        return Err(ret.into());
    } else if buf.len() > len {
        return Err(SysError::LengthNotEnough);
    }
    Ok(buf)
}

pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_DEBUG);
}
