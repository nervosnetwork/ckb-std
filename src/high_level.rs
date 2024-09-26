use crate::ckb_constants::*;
use crate::error::SysError;
use crate::syscalls;
use alloc::{ffi::CString, string::String, vec, vec::Vec};
use ckb_types::{core::ScriptHashType, packed::*, prelude::*};
use core::convert::Infallible;
use core::ffi::CStr;
use core::fmt::Write;

/// Default buffer size, it is used to load data from syscall.
/// The default value is set to 256, which will be enough for most cases:
/// * load a `Script`, the typical size is 73 ~ 86
/// * load a `CellOutput`, the typical size is 97 ~ 195
pub const BUF_SIZE: usize = 256;

/// Load tx hash
///
/// Return the tx hash or a syscall error
///
/// # Example
///
/// ```
/// let tx_hash = load_tx_hash().unwrap();
/// ```
pub fn load_tx_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

/// Load script hash
///
/// Return the script hash or a syscall error
///
/// # Example
///
/// ```
/// let script_hash = load_script_hash().unwrap();
/// ```
pub fn load_script_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_script_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

/// Common method to fully load data from syscall
fn load_data<F: Fn(&mut [u8], usize) -> Result<usize, SysError>>(
    syscall: F,
) -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    match syscall(&mut buf, 0) {
        Ok(len) => Ok(buf[..len].to_vec()),
        Err(SysError::LengthNotEnough(actual_size)) => {
            let mut data = vec![0; actual_size];
            let loaded_len = buf.len();
            data[..loaded_len].copy_from_slice(&buf);
            let len = syscall(&mut data[loaded_len..], loaded_len)?;
            debug_assert_eq!(len + loaded_len, actual_size);
            Ok(data)
        }
        Err(err) => Err(err),
    }
}

/// Load cell
///
/// Return the cell or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let cell_output = load_cell(0, Source::Input).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn load_cell(index: usize, source: Source) -> Result<CellOutput, SysError> {
    let data = load_data(|buf, offset| syscalls::load_cell(buf, offset, index, source))?;

    match CellOutputReader::verify(&data, false) {
        Ok(()) => Ok(CellOutput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load input
///
/// Return the input or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let input = load_input(0, Source::Input).unwrap();
/// ```
pub fn load_input(index: usize, source: Source) -> Result<CellInput, SysError> {
    let mut data = [0u8; CellInput::TOTAL_SIZE];
    syscalls::load_input(&mut data, 0, index, source)?;

    match CellInputReader::verify(&data, false) {
        Ok(()) => Ok(CellInput::new_unchecked(data.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load header
///
/// Return the header or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let header = load_header(0, Source::HeaderDep).unwrap();
/// ```
pub fn load_header(index: usize, source: Source) -> Result<Header, SysError> {
    let mut data = [0u8; Header::TOTAL_SIZE];
    syscalls::load_header(&mut data, 0, index, source)?;

    match HeaderReader::verify(&data, false) {
        Ok(()) => Ok(Header::new_unchecked(data.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load witness
///
/// Return the witness or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let witness = load_witness(0, Source::Input).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn load_witness(index: usize, source: Source) -> Result<Vec<u8>, SysError> {
    load_data(|buf, offset| syscalls::load_witness(buf, offset, index, source))
}

/// Load witness args
///
/// Return the witness args or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let witness_args = load_witness_args(0, Source::Input).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn load_witness_args(index: usize, source: Source) -> Result<WitnessArgs, SysError> {
    let data = load_data(|buf, offset| syscalls::load_witness(buf, offset, index, source))?;

    match WitnessArgsReader::verify(&data, false) {
        Ok(()) => Ok(WitnessArgs::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load transaction
///
/// Return the transaction or a syscall error
///
/// # Example
///
/// ```
/// let tx = load_transaction().unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn load_transaction() -> Result<Transaction, SysError> {
    let data = load_data(|buf, offset| syscalls::load_transaction(buf, offset))?;

    match TransactionReader::verify(&data, false) {
        Ok(()) => Ok(Transaction::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell capacity
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let capacity = syscalls::load_cell_capacity(index, source).unwrap();
/// ```
pub fn load_cell_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::Capacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load cell occupied capacity
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let occupied_capacity = load_cell_occupied_capacity(index, source).unwrap();
/// ```
pub fn load_cell_occupied_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len =
        syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::OccupiedCapacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load cell data hash
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let data_hash = load_cell_data_hash(index, source).unwrap();
/// ```
pub fn load_cell_data_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::DataHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

/// Load cell lock hash
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let lock_hash = load_cell_lock_hash(index, source).unwrap();
/// ```
pub fn load_cell_lock_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::LockHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

/// Load cell type hash
///
/// return None if the cell has no type
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let type_hash = load_cell_type_hash(index, source).unwrap().unwrap();
/// ```
pub fn load_cell_type_hash(index: usize, source: Source) -> Result<Option<[u8; 32]>, SysError> {
    let mut buf = [0u8; 32];
    match syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::TypeHash) {
        Ok(len) => {
            debug_assert_eq!(len, buf.len());
            Ok(Some(buf))
        }
        Err(SysError::ItemMissing) => Ok(None),
        Err(err) => Err(err),
    }
}

/// Load cell lock
///
/// Return the lock script or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let lock = load_cell_lock(index, source).unwrap();
/// ```
pub fn load_cell_lock(index: usize, source: Source) -> Result<Script, SysError> {
    let data = load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Lock)
    })?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell type
///
/// Return the type script or a syscall error, return None if the cell has no type
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let type_script = load_cell_type(index, source).unwrap().unwrap();
/// ```
pub fn load_cell_type(index: usize, source: Source) -> Result<Option<Script>, SysError> {
    let data = match load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Type)
    }) {
        Ok(data) => data,
        Err(SysError::ItemMissing) => return Ok(None),
        Err(err) => return Err(err),
    };

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Some(Script::new_unchecked(data.into()))),
        Err(_err) => Err(SysError::Encoding),
    }
}

// Load header epoch number
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_number = load_header_epoch_number(index, source).unwrap();
/// ```
pub fn load_header_epoch_number(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochNumber)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load header epoch start block number
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_start_block_number = load_header_epoch_start_block_number(index, source).unwrap();
/// ```
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

/// Load header epoch length
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_length = load_header_epoch_length(index, source).unwrap();
/// ```
pub fn load_header_epoch_length(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochLength)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load input since
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let since = load_input_since(index, source).unwrap();
/// ```
pub fn load_input_since(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_input_by_field(&mut buf, 0, index, source, InputField::Since)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load input out point
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let out_point = load_input_out_point(index, source).unwrap();
/// ```
pub fn load_input_out_point(index: usize, source: Source) -> Result<OutPoint, SysError> {
    let mut data = [0u8; OutPoint::TOTAL_SIZE];
    syscalls::load_input_by_field(&mut data, 0, index, source, InputField::OutPoint)?;

    match OutPointReader::verify(&data, false) {
        Ok(()) => Ok(OutPoint::new_unchecked(data.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell data
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let data = load_cell_data(index, source).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn load_cell_data(index: usize, source: Source) -> Result<Vec<u8>, SysError> {
    load_data(|buf, offset| syscalls::load_cell_data(buf, offset, index, source))
}

/// Load script
///
/// # Example
///
/// ```
/// let script = load_script().unwrap();
/// ```
pub fn load_script() -> Result<Script, SysError> {
    let data = load_data(|buf, offset| syscalls::load_script(buf, offset))?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// QueryIter
///
/// A advanced iterator to manipulate cells/inputs/headers/witnesses
///
/// # Example
///
/// ```
/// use high_level::load_cell_capacity;
/// // calculate all inputs capacity
/// let inputs_capacity = QueryIter::new(load_cell_capacity, Source::Input)
/// .map(|capacity| capacity.unwrap_or(0))
/// .sum::<u64>();
///
/// // calculate all outputs capacity
/// let outputs_capacity = QueryIter::new(load_cell_capacity, Source::Output)
/// .map(|capacity| capacity.unwrap_or(0))
/// .sum::<u64>();
///
/// assert_eq!(inputs_capacity, outputs_capacity);
/// ```
pub struct QueryIter<F> {
    query_fn: F,
    index: usize,
    source: Source,
}

impl<F> QueryIter<F> {
    /// new
    ///
    /// # Arguments
    ///
    /// * `query_fn` - A high level query function, which accept `(index, source)` as args and
    /// returns Result<T, SysError>. Examples: `load_cell`, `load_cell_data`,`load_witness_args`, `load_input`, `load_header`, ...
    /// * `source` - source
    ///
    /// # Example
    ///
    /// ```
    /// use high_level::load_cell;
    /// // iterate all inputs cells
    /// let iter = QueryIter::new(load_cell, Source::Input)
    /// ```
    pub fn new(query_fn: F, source: Source) -> Self {
        QueryIter {
            query_fn,
            index: 0,
            source,
        }
    }
}

impl<T, F: Fn(usize, Source) -> Result<T, SysError>> Iterator for QueryIter<F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.query_fn)(self.index, self.source) {
            Ok(item) => {
                self.index += 1;
                Some(item)
            }
            Err(SysError::IndexOutOfBound) => None,
            Err(err) => {
                panic!("QueryIter error {:?}", err);
            }
        }
    }
}

/// Find cell by data_hash
///
/// Iterate and find the cell which data hash equals `data_hash`,
/// return the index of the first cell we found, otherwise return None.
///
pub fn find_cell_by_data_hash(data_hash: &[u8], source: Source) -> Result<Option<usize>, SysError> {
    let mut buf = [0u8; 32];
    for i in 0.. {
        let len = match syscalls::load_cell_by_field(&mut buf, 0, i, source, CellField::DataHash) {
            Ok(len) => len,
            Err(SysError::IndexOutOfBound) => break,
            Err(err) => return Err(err),
        };
        debug_assert_eq!(len, buf.len());
        if data_hash == &buf[..] {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

/// Look for a dep cell with specific code hash, code_hash should be a buffer
/// with 32 bytes.
///
pub fn look_for_dep_with_hash2(
    code_hash: &[u8],
    hash_type: ScriptHashType,
) -> Result<usize, SysError> {
    let field = match hash_type {
        ScriptHashType::Type => CellField::TypeHash,
        _ => CellField::DataHash,
    };
    let mut current: usize = 0;
    loop {
        let mut buf = [0u8; 32];
        match syscalls::load_cell_by_field(&mut buf, 0, current, Source::CellDep, field) {
            Ok(len) => {
                debug_assert_eq!(len, buf.len());
                if buf == code_hash {
                    return Ok(current);
                }
            }
            Err(SysError::ItemMissing) => {}
            Err(SysError::IndexOutOfBound) => {
                return Err(SysError::IndexOutOfBound);
            }
            Err(err) => {
                return Err(err);
            }
        }
        current += 1;
    }
}

pub fn look_for_dep_with_data_hash(data_hash: &[u8]) -> Result<usize, SysError> {
    look_for_dep_with_hash2(data_hash, ScriptHashType::Data)
}

pub fn encode_hex(data: &[u8]) -> CString {
    let mut s = String::with_capacity(data.len() * 2);
    for &b in data {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    CString::new(s).unwrap()
}

pub fn decode_hex(data: &CStr) -> Result<Vec<u8>, SysError> {
    let data = data.to_str().unwrap();
    if data.len() & 1 != 0 {
        return Err(SysError::Encoding);
    }
    (0..data.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&data[i..i + 2], 16).map_err(|_| SysError::Encoding))
        .collect()
}

/// Exec a cell in cell dep.
///
/// # Arguments
///
/// * `code_hash` - the code hash to search cell in cell deps.
/// * `hash_type` - the hash type to search cell in cell deps.
/// * `argv` - subprocess arguments. In most cases two types of parameters can be accepted:
///            - if the parameter you want to pass can be represented by a string:
///              - CStr::from_bytes_with_nul(b"arg0\0").unwrap();
///              - CString::new("arg0").unwrap().as_c_str();
///            - if you want to pass a piece of bytes data, you may encode it to hexadecimal string or other format:
///              - high_level::encode_hex(&vec![0xff, 0xfe, 0xfd]);
pub fn exec_cell(
    code_hash: &[u8],
    hash_type: ScriptHashType,
    argv: &[&CStr],
) -> Result<Infallible, SysError> {
    #[cfg(not(feature = "native-simulator"))]
    {
        let index = look_for_dep_with_hash2(code_hash, hash_type)?;
        let ret = syscalls::exec(index, Source::CellDep, 0, 0, argv);
        let err = match ret {
            1 => SysError::IndexOutOfBound,
            2 => SysError::ItemMissing,
            r => SysError::Unknown(r),
        };
        Err(err)
    }
    #[cfg(feature = "native-simulator")]
    syscalls::exec_cell(code_hash, hash_type, argv)
}

/// Spawn a cell in cell dep.
///
/// # Arguments
///
/// * `code_hash` - the code hash to search cell in cell deps.
/// * `hash_type` - the hash type to search cell in cell deps.
/// * `argv` - subprocess arguments. In most cases two types of parameters can be accepted:
///            - if the parameter you want to pass can be represented by a string:
///              - CStr::from_bytes_with_nul(b"arg0\0").unwrap();
///              - CString::new("arg0").unwrap().as_c_str();
///            - if you want to pass a piece of bytes data, you may encode it to hexadecimal string or other format:
///              - high_level::encode_hex(&vec![0xff, 0xfe, 0xfd]);
/// * `inherited_fds` - the fd list to be passed to the child process.
pub fn spawn_cell(
    code_hash: &[u8],
    hash_type: ScriptHashType,
    argv: &[&CStr],
    inherited_fds: &[u64],
) -> Result<u64, SysError> {
    let index = look_for_dep_with_hash2(code_hash, hash_type)?;
    let argc = argv.len();
    let mut process_id: u64 = 0;
    let argv_ptr: Vec<*const i8> = argv.iter().map(|e| e.as_ptr()).collect();
    let mut spgs = syscalls::SpawnArgs {
        argc: argc as u64,
        argv: argv_ptr.as_ptr(),
        process_id: &mut process_id as *mut u64,
        inherited_fds: inherited_fds.as_ptr(),
    };
    syscalls::spawn(index, Source::CellDep, 0, 0, &mut spgs)?;
    Ok(process_id)
}
