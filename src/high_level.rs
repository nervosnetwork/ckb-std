use crate::ckb_constants::*;
use crate::error::SysError;
use crate::syscalls;
use alloc::vec::Vec;
use ckb_types::{packed::*, prelude::*};

pub const BUF_SIZE: usize = 1024;

pub fn load_tx_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

pub fn load_script_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_script_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

/// Common method to load all data from syscall
fn load_data<F: Fn(&mut [u8], usize) -> Result<usize, SysError>>(
    syscall: F,
) -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    match syscall(&mut buf, 0) {
        Ok(len) => Ok(buf[..len].to_vec()),
        Err(SysError::LengthNotEnough(actual_size)) => {
            let mut data = Vec::with_capacity(actual_size);
            let loaded_len = buf.len();
            data[..loaded_len].copy_from_slice(&buf);
            let len = syscall(&mut data[loaded_len..], loaded_len)?;
            debug_assert_eq!(len + loaded_len, actual_size);
            Ok(data)
        }
        Err(err) => Err(err),
    }
}

pub fn load_cell(index: usize, source: Source) -> Result<CellOutput, SysError> {
    let data = load_data(|buf, offset| syscalls::load_cell(buf, offset, index, source))?;

    match CellOutputReader::verify(&data, false) {
        Ok(()) => Ok(CellOutput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

pub fn load_input(index: usize, source: Source) -> Result<CellInput, SysError> {
    let data = load_data(|buf, offset| syscalls::load_input(buf, offset, index, source))?;

    match CellInputReader::verify(&data, false) {
        Ok(()) => Ok(CellInput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

pub fn load_header(index: usize, source: Source) -> Result<Header, SysError> {
    let data = load_data(|buf, offset| syscalls::load_header(buf, offset, index, source))?;

    match HeaderReader::verify(&data, false) {
        Ok(()) => Ok(Header::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

pub fn load_witness_args(index: usize, source: Source) -> Result<WitnessArgs, SysError> {
    let data = load_data(|buf, offset| syscalls::load_witness(buf, offset, index, source))?;

    match WitnessArgsReader::verify(&data, false) {
        Ok(()) => Ok(WitnessArgs::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

pub fn load_transaction() -> Result<Transaction, SysError> {
    let data = load_data(|buf, offset| syscalls::load_transaction(buf, offset))?;

    match TransactionReader::verify(&data, false) {
        Ok(()) => Ok(Transaction::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

// Load cell by fields

pub fn load_cell_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::Capacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

pub fn load_cell_occupied_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len =
        syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::OccupiedCapacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

pub fn load_cell_data_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::DataHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

pub fn load_cell_lock_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::LockHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

pub fn load_cell_type_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::TypeHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

pub fn load_cell_lock(index: usize, source: Source) -> Result<Script, SysError> {
    let data = load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Lock)
    })?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

pub fn load_cell_type(index: usize, source: Source) -> Result<Script, SysError> {
    let data = load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Type)
    })?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

// Load header by field

pub fn load_header_epoch_number(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochNumber)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

pub fn load_header_epoch_start_block_number(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(
        &mut buf,
        0,
        index,
        source,
        HeaderField::EpochStartBlockNumber,
    )?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

pub fn load_header_epoch_length(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochLength)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

// Load input by field

pub fn load_input_since(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_input_by_field(&mut buf, 0, index, source, InputField::Since)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

pub fn load_input_out_point(index: usize, source: Source) -> Result<OutPoint, SysError> {
    let mut buf = [0u8; 36];
    let len = syscalls::load_input_by_field(&mut buf, 0, index, source, InputField::OutPoint)?;
    debug_assert_eq!(len, buf.len());
    match OutPointReader::verify(&buf, false) {
        Ok(()) => Ok(OutPoint::new_unchecked(buf.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell data, read cell data
pub fn load_cell_data(index: usize, source: Source) -> Result<Vec<u8>, SysError> {
    load_data(|buf, offset| syscalls::load_cell_data(buf, offset, index, source))
}

/// Load script
pub fn load_script() -> Result<Script, SysError> {
    let data = load_data(|buf, offset| syscalls::load_script(buf, offset))?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}
