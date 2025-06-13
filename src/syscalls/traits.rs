use crate::{
    ckb_constants::{self as consts, CellField, HeaderField, InputField, Source},
    error::SysError,
    syscalls::internal::SpawnArgs,
};
use alloc::string::String;
use core::ffi::CStr;

/// This trait serves several purposes:
///
/// * Current ckb-std syscall implementations were written at different
///   time with different mindsets, and accumulated throughout the years.
///   As a result, it presents certain inconsistencies and issues. Just to
///   name a few:
///     + The newly introduced read syscall interprets its return values
///       differently from old `load_*` syscalls. However they share the
///       same return value type, which can be a source of confusion.
///     + Some signature design could use a little work: for example, spawn
///       sets returned process ID both in one of the mutable argument, and
///       also in its return values. This is really duplicate information that
///       can be revisited. In addition, the `argv` data structured, used by
///       both spawn and exec, are passed differently in both syscalls. In
///       hindset, maybe we don't need to expose the C style `SpawnArgs` structure
///       in Rust APIs, but keep it as an internal data structure.
///     + The return value of inherited_fds syscall is completely ignored,
///       only the length of written fds is returned.
/// * New features such as native simulators, or fuzzing require customized
///   syscall implementations. There is no proper way we can customize a CKB
///   script for alternative syscall implementations.
///
/// On the other hand, compatibility remains a consideration, it might not be
/// possible to alter current syscall implementations, which might affect real
/// usage.
///
/// This trait aims to provide a new solution, where all CKB syscalls can be
/// provided by a single trait. It also attempts to clear and unify syscall
/// APIs, in a clear and easy to understand fashion.
pub trait SyscallImpls {
    fn debug(&self, s: &CStr);
    fn exit(&self, code: i8) -> !;
    fn load_cell(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult;
    fn load_cell_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: CellField,
    ) -> IoResult;
    fn load_cell_code(
        &self,
        buf_ptr: *mut u8,
        len: usize,
        content_offset: usize,
        content_size: usize,
        index: usize,
        source: Source,
    ) -> Result<(), Error>;
    fn load_cell_data(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
    ) -> IoResult;
    fn load_header(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult;
    fn load_header_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: HeaderField,
    ) -> IoResult;
    fn load_input(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult;
    fn load_input_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: InputField,
    ) -> IoResult;
    fn load_script(&self, buf: &mut [u8], offset: usize) -> IoResult;
    fn load_script_hash(&self, buf: &mut [u8], offset: usize) -> IoResult;
    fn load_transaction(&self, buf: &mut [u8], offset: usize) -> IoResult;
    fn load_tx_hash(&self, buf: &mut [u8], offset: usize) -> IoResult;
    fn load_witness(&self, buf: &mut [u8], offset: usize, index: usize, source: Source)
    -> IoResult;

    fn vm_version(&self) -> u64;
    fn current_cycles(&self) -> u64;
    fn exec(
        &self,
        index: usize,
        source: Source,
        place: usize,
        bounds: usize,
        argv: &[&CStr],
    ) -> Result<(), Error>;

    /// Spawned process ID is returned when the syscall succeeds
    fn spawn(
        &self,
        index: usize,
        source: Source,
        place: usize,
        bounds: usize,
        argv: &[&CStr],
        inherited_fds: &[u64],
    ) -> Result<u64, Error>;
    fn pipe(&self) -> Result<(u64, u64), Error>;
    /// Number of available fds is returned when the syscall succeeds, which
    /// can be bigger than the length of the passed argument `fds` slice
    fn inherited_fds(&self, fds: &mut [u64]) -> Result<usize, Error>;
    /// Number of read bytes is returned when the syscall succeeds. Note
    /// this syscall works unlike the `load_*` syscalls, it only returns
    /// the number of bytes read to passed buffer. The syscall has no way
    /// of knowing how many bytes are availble to read.
    fn read(&self, fd: u64, buffer: &mut [u8]) -> Result<usize, Error>;
    /// Number of written bytes is returned when the syscall succeeds.
    fn write(&self, fd: u64, buffer: &[u8]) -> Result<usize, Error>;
    fn close(&self, fd: u64) -> Result<(), Error>;
    /// Exit code of waited process is returned when the syscall succeeds.
    fn wait(&self, pid: u64) -> Result<i8, Error>;
    fn process_id(&self) -> u64;
    fn load_block_extension(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
    ) -> IoResult;

    fn debug_s(&self, mut s: String) {
        s.push('\0');
        let bytes = s.into_bytes();
        let c_str = CStr::from_bytes_until_nul(&bytes).unwrap();
        self.debug(c_str)
    }
}

pub trait SyscallExecutor {
    fn syscall(&self, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, n: u64) -> u64;
}

/// A default SyscallImpls implementation in case you feel like the new
/// interface
pub struct DefaultSyscallImpls<E: SyscallExecutor>(E);

impl<E: SyscallExecutor> DefaultSyscallImpls<E> {
    pub fn new(e: E) -> Self {
        Self(e)
    }

    fn syscall_load(
        &self,
        buf: &mut [u8],
        offset: usize,
        a3: u64,
        a4: u64,
        a5: u64,
        syscall_num: u64,
    ) -> IoResult {
        let mut actual_data_len: u64 = buf.len() as u64;
        let len_ptr: *mut u64 = &mut actual_data_len;
        let ret = self.0.syscall(
            buf.as_ptr() as u64,
            len_ptr as u64,
            offset as u64,
            a3,
            a4,
            a5,
            syscall_num,
        );
        match ret {
            0 => {
                if actual_data_len > buf.len() as u64 {
                    IoResult::PartialLoaded {
                        loaded: buf.len(),
                        available: actual_data_len as usize,
                    }
                } else {
                    IoResult::FullyLoaded(actual_data_len as usize)
                }
            }
            _ => IoResult::Error(ret.try_into().unwrap()),
        }
    }
}

impl<E: SyscallExecutor> SyscallImpls for DefaultSyscallImpls<E> {
    fn debug(&self, s: &CStr) {
        self.0
            .syscall(s.as_ptr() as u64, 0, 0, 0, 0, 0, consts::SYS_DEBUG);
    }

    fn exit(&self, code: i8) -> ! {
        self.0.syscall(code as u64, 0, 0, 0, 0, 0, consts::SYS_EXIT);
        unreachable!()
    }

    fn load_cell(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_CELL,
        )
    }

    fn load_cell_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: CellField,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            field as u64,
            consts::SYS_LOAD_CELL_BY_FIELD,
        )
    }

    fn load_cell_code(
        &self,
        buf_ptr: *mut u8,
        len: usize,
        content_offset: usize,
        content_size: usize,
        index: usize,
        source: Source,
    ) -> Result<(), Error> {
        build_result(self.0.syscall(
            buf_ptr as u64,
            len as u64,
            content_offset as u64,
            content_size as u64,
            index as u64,
            source as u64,
            consts::SYS_LOAD_CELL_DATA_AS_CODE,
        ))
    }

    fn load_cell_data(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_CELL_DATA,
        )
    }

    fn load_header(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_HEADER,
        )
    }

    fn load_header_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: HeaderField,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            field as u64,
            consts::SYS_LOAD_HEADER_BY_FIELD,
        )
    }

    fn load_input(&self, buf: &mut [u8], offset: usize, index: usize, source: Source) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_INPUT,
        )
    }

    fn load_input_by_field(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
        field: InputField,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            field as u64,
            consts::SYS_LOAD_INPUT_BY_FIELD,
        )
    }

    fn load_script(&self, buf: &mut [u8], offset: usize) -> IoResult {
        self.syscall_load(buf, offset, 0, 0, 0, consts::SYS_LOAD_SCRIPT)
    }

    fn load_script_hash(&self, buf: &mut [u8], offset: usize) -> IoResult {
        self.syscall_load(buf, offset, 0, 0, 0, consts::SYS_LOAD_SCRIPT_HASH)
    }

    fn load_transaction(&self, buf: &mut [u8], offset: usize) -> IoResult {
        self.syscall_load(buf, offset, 0, 0, 0, consts::SYS_LOAD_TRANSACTION)
    }

    fn load_tx_hash(&self, buf: &mut [u8], offset: usize) -> IoResult {
        self.syscall_load(buf, offset, 0, 0, 0, consts::SYS_LOAD_TX_HASH)
    }

    fn load_witness(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_WITNESS,
        )
    }

    fn vm_version(&self) -> u64 {
        self.0.syscall(0, 0, 0, 0, 0, 0, consts::SYS_VM_VERSION)
    }

    fn current_cycles(&self) -> u64 {
        self.0.syscall(0, 0, 0, 0, 0, 0, consts::SYS_CURRENT_CYCLES)
    }

    fn exec(
        &self,
        index: usize,
        source: Source,
        place: usize,
        bounds: usize,
        argv: &[&CStr],
    ) -> Result<(), Error> {
        let argv_ptr: alloc::vec::Vec<*const i8> =
            argv.iter().map(|e| e.as_ptr() as *const i8).collect();

        build_result(self.0.syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds as u64,
            argv.len() as u64,
            argv_ptr.as_ptr() as u64,
            consts::SYS_EXEC,
        ))
    }

    fn spawn(
        &self,
        index: usize,
        source: Source,
        place: usize,
        bounds: usize,
        argv: &[&CStr],
        inherited_fds: &[u64],
    ) -> Result<u64, Error> {
        let mut fds_with_terminator = alloc::vec![0; inherited_fds.len() + 1];
        fds_with_terminator[0..inherited_fds.len()].copy_from_slice(inherited_fds);

        let argv_ptr: alloc::vec::Vec<*const i8> =
            argv.iter().map(|e| e.as_ptr() as *const i8).collect();

        let mut process_id = 0;
        let mut spgs = SpawnArgs {
            argc: argv.len() as u64,
            argv: argv_ptr.as_ptr(),
            process_id: &mut process_id,
            inherited_fds: fds_with_terminator.as_ptr(),
        };

        build_result(self.0.syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds as u64,
            &mut spgs as *mut _ as u64,
            0,
            consts::SYS_SPAWN,
        ))
        .map(|_| process_id)
    }

    fn pipe(&self) -> Result<(u64, u64), Error> {
        let mut fds: [u64; 2] = [0, 0];
        build_result(
            self.0
                .syscall(fds.as_mut_ptr() as u64, 0, 0, 0, 0, 0, consts::SYS_PIPE),
        )
        .map(|_| (fds[0], fds[1]))
    }

    fn inherited_fds(&self, fds: &mut [u64]) -> Result<usize, Error> {
        let mut length: u64 = fds.len() as u64;
        build_result(self.0.syscall(
            fds.as_mut_ptr() as u64,
            &mut length as *mut _ as u64,
            0,
            0,
            0,
            0,
            consts::SYS_INHERITED_FDS,
        ))
        .map(|_| length as usize)
    }

    fn read(&self, fd: u64, buffer: &mut [u8]) -> Result<usize, Error> {
        let mut length: u64 = buffer.len() as u64;
        build_result(self.0.syscall(
            fd,
            buffer.as_mut_ptr() as u64,
            &mut length as *mut _ as u64,
            0,
            0,
            0,
            consts::SYS_READ,
        ))
        .map(|_| length as usize)
    }

    fn write(&self, fd: u64, buffer: &[u8]) -> Result<usize, Error> {
        let mut length: u64 = buffer.len() as u64;
        build_result(self.0.syscall(
            fd,
            buffer.as_ptr() as u64,
            &mut length as *mut _ as u64,
            0,
            0,
            0,
            consts::SYS_WRITE,
        ))
        .map(|_| length as usize)
    }

    fn close(&self, fd: u64) -> Result<(), Error> {
        build_result(self.0.syscall(fd, 0, 0, 0, 0, 0, consts::SYS_CLOSE))
    }

    fn wait(&self, pid: u64) -> Result<i8, Error> {
        let mut code: u64 = u64::MAX;
        build_result(self.0.syscall(
            pid,
            &mut code as *mut _ as u64,
            0,
            0,
            0,
            0,
            consts::SYS_WAIT,
        ))
        .map(|_| code as i8)
    }

    fn process_id(&self) -> u64 {
        self.0.syscall(0, 0, 0, 0, 0, 0, consts::SYS_PROCESS_ID)
    }

    fn load_block_extension(
        &self,
        buf: &mut [u8],
        offset: usize,
        index: usize,
        source: Source,
    ) -> IoResult {
        self.syscall_load(
            buf,
            offset,
            index as u64,
            source as u64,
            0,
            consts::SYS_LOAD_BLOCK_EXTENSION,
        )
    }
}

/// Error defined here differs from SysError: it only captures true CKB
/// errors. It is not considered an error when a partial loading function
/// reads part, but not all of the data.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    IndexOutOfBound,
    ItemMissing,
    SliceOutOfBound,
    WrongFormat,
    WaitFailure,
    InvalidFd,
    OtherEndClosed,
    MaxVmsSpawned,
    MaxFdsCreated,
    Other(u64),
}

impl From<Error> for u64 {
    fn from(e: Error) -> u64 {
        match e {
            Error::IndexOutOfBound => 1,
            Error::ItemMissing => 2,
            Error::SliceOutOfBound => 3,
            Error::WrongFormat => 4,
            Error::WaitFailure => 5,
            Error::InvalidFd => 6,
            Error::OtherEndClosed => 7,
            Error::MaxVmsSpawned => 8,
            Error::MaxFdsCreated => 9,
            Error::Other(e) => e,
        }
    }
}

pub fn build_result(v: u64) -> Result<(), Error> {
    if v == 0 {
        Ok(())
    } else {
        Err(v.try_into().unwrap())
    }
}

impl TryFrom<u64> for Error {
    type Error = &'static str;

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            0 => Err("Error cannot be zero!"),
            1 => Ok(Error::IndexOutOfBound),
            2 => Ok(Error::ItemMissing),
            3 => Ok(Error::SliceOutOfBound),
            4 => Ok(Error::WrongFormat),
            5 => Ok(Error::WaitFailure),
            6 => Ok(Error::InvalidFd),
            7 => Ok(Error::OtherEndClosed),
            8 => Ok(Error::MaxVmsSpawned),
            9 => Ok(Error::MaxFdsCreated),
            _ => Ok(Error::Other(v)),
        }
    }
}

/// IoResult captures response from partial loading function.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IoResult {
    FullyLoaded(usize),
    PartialLoaded { loaded: usize, available: usize },
    Error(Error),
}

impl IoResult {
    pub fn to_result(&self) -> Result<(), Error> {
        match self {
            IoResult::FullyLoaded(_) => Ok(()),
            IoResult::PartialLoaded { .. } => Ok(()),
            IoResult::Error(e) => Err(*e),
        }
    }

    pub fn loaded(&self) -> Option<usize> {
        match self {
            IoResult::FullyLoaded(l) => Some(*l),
            IoResult::PartialLoaded { loaded, .. } => Some(*loaded),
            IoResult::Error(_) => None,
        }
    }

    pub fn available(&self) -> Option<usize> {
        match self {
            IoResult::FullyLoaded(l) => Some(*l),
            IoResult::PartialLoaded { available, .. } => Some(*available),
            IoResult::Error(_) => None,
        }
    }
}

impl From<Error> for IoResult {
    fn from(e: Error) -> IoResult {
        IoResult::Error(e)
    }
}

impl From<IoResult> for Result<usize, SysError> {
    fn from(result: IoResult) -> Result<usize, SysError> {
        match result {
            IoResult::FullyLoaded(l) => Ok(l),
            IoResult::PartialLoaded { available, .. } => Err(SysError::LengthNotEnough(available)),
            IoResult::Error(e) => Err(e.into()),
        }
    }
}

impl From<Error> for SysError {
    fn from(e: Error) -> SysError {
        match e {
            Error::IndexOutOfBound => SysError::IndexOutOfBound,
            Error::ItemMissing => SysError::ItemMissing,
            Error::SliceOutOfBound => SysError::Encoding,
            Error::WrongFormat => SysError::Unknown(4),
            Error::WaitFailure => SysError::WaitFailure,
            Error::InvalidFd => SysError::InvalidFd,
            Error::OtherEndClosed => SysError::OtherEndClosed,
            Error::MaxVmsSpawned => SysError::MaxVmsSpawned,
            Error::MaxFdsCreated => SysError::MaxFdsCreated,
            Error::Other(e) => SysError::Unknown(e),
        }
    }
}
