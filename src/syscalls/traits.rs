use crate::{
    ckb_constants::{self as consts, CellField, HeaderField, InputField, Source},
    error::SysError,
    syscalls::internal::SpawnArgs,
};
use alloc::{string::String, vec::Vec};
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
    /// There are 2 ways you can implement this trait: you can either implement
    /// this generic syscall function, where you detect the syscall by the last
    /// `n` field, or you can implement each invididual syscall in a type-safe
    /// way. Dummy implementations are provided for each trait method so one can
    /// override only the needed ones.
    fn syscall(&self, _a0: u64, _a1: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _n: u64) -> u64 {
        panic!("Default syscall function is not yet implemented!")
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
        let ret = self.syscall(
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

    fn debug(&self, s: &CStr) {
        self.syscall(s.as_ptr() as u64, 0, 0, 0, 0, 0, consts::SYS_DEBUG);
    }
    fn exit(&self, code: i8) -> ! {
        self.syscall(code as u64, 0, 0, 0, 0, 0, consts::SYS_EXIT);
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
    ) -> Result<Vec<MemoryPageResult>, Error> {
        build_result(self.syscall(
            buf_ptr as u64,
            len as u64,
            content_offset as u64,
            content_size as u64,
            index as u64,
            source as u64,
            consts::SYS_LOAD_CELL_DATA_AS_CODE,
        ))
        .and_then(|_| Ok(Vec::new()))
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
        self.syscall(0, 0, 0, 0, 0, 0, consts::SYS_VM_VERSION)
    }
    fn current_cycles(&self) -> u64 {
        self.syscall(0, 0, 0, 0, 0, 0, consts::SYS_CURRENT_CYCLES)
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

        build_result(self.syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds as u64,
            argv.len() as u64,
            argv_ptr.as_ptr() as u64,
            consts::SYS_EXEC,
        ))
    }

    /// Spawned process ID is returned when the syscall succeeds
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

        build_result(self.syscall(
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
        build_result(self.syscall(fds.as_mut_ptr() as u64, 0, 0, 0, 0, 0, consts::SYS_PIPE))
            .map(|_| (fds[0], fds[1]))
    }
    /// Number of available fds is returned when the syscall succeeds, which
    /// can be bigger than the length of the passed argument `fds` slice
    fn inherited_fds(&self, fds: &mut [u64]) -> Result<usize, Error> {
        let mut length: u64 = fds.len() as u64;
        build_result(self.syscall(
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
    /// Number of read bytes is returned when the syscall succeeds. Note
    /// this syscall works unlike the `load_*` syscalls, it only returns
    /// the number of bytes read to passed buffer. The syscall has no way
    /// of knowing how many bytes are availble to read.
    fn read(&self, fd: u64, buffer: &mut [u8]) -> Result<usize, Error> {
        let mut length: u64 = buffer.len() as u64;
        build_result(self.syscall(
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
    /// Number of written bytes is returned when the syscall succeeds.
    fn write(&self, fd: u64, buffer: &[u8]) -> Result<usize, Error> {
        let mut length: u64 = buffer.len() as u64;
        build_result(self.syscall(
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
        build_result(self.syscall(fd, 0, 0, 0, 0, 0, consts::SYS_CLOSE))
    }
    /// Exit code of waited process is returned when the syscall succeeds.
    fn wait(&self, pid: u64) -> Result<i8, Error> {
        let mut code: u64 = u64::MAX;
        build_result(self.syscall(
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
        self.syscall(0, 0, 0, 0, 0, 0, consts::SYS_PROCESS_ID)
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

    fn debug_s(&self, mut s: String) {
        s.push('\0');
        let bytes = s.into_bytes();
        let c_str = CStr::from_bytes_until_nul(&bytes).unwrap();
        self.debug(c_str)
    }
}

/// This is the inverse of DefaultSyscallImpls: given a general syscall function,
/// we map the syscalls to a SyscallImpls trait impl. This way we are taking care
/// of the unsafe part for you, where in Rust you can just deal with SyscallImpls.
pub fn syscall_to_impls<S: SyscallImpls + ?Sized>(
    impls: &S,
    n: u64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> u64 {
    match n {
        consts::SYS_DEBUG => {
            impls.debug(unsafe { CStr::from_ptr(a0 as *const _) });
            0
        }
        consts::SYS_EXIT => impls.exit(a0 as i8),
        consts::SYS_LOAD_CELL => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_cell(buf, a2 as usize, a3 as usize, source)
        }),
        consts::SYS_LOAD_CELL_BY_FIELD => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            let field: CellField = a5.try_into().expect("parse cell field");
            impls.load_cell_by_field(buf, a2 as usize, a3 as usize, source, field)
        }),
        consts::SYS_LOAD_CELL_DATA_AS_CODE => {
            let source: Source = a5.try_into().expect("parse source");
            match impls.load_cell_code(
                a0 as *mut u8,
                a1 as usize,
                a2 as usize,
                a3 as usize,
                a4 as usize,
                source,
            ) {
                Ok(_) => 0,
                Err(e) => e.into(),
            }
        }
        consts::SYS_LOAD_CELL_DATA => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_cell_data(buf, a2 as usize, a3 as usize, source)
        }),
        consts::SYS_LOAD_HEADER => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_header(buf, a2 as usize, a3 as usize, source)
        }),
        consts::SYS_LOAD_HEADER_BY_FIELD => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            let field: HeaderField = a5.try_into().expect("parse header field");
            impls.load_header_by_field(buf, a2 as usize, a3 as usize, source, field)
        }),
        consts::SYS_LOAD_INPUT => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_input(buf, a2 as usize, a3 as usize, source)
        }),
        consts::SYS_LOAD_INPUT_BY_FIELD => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            let field: InputField = a5.try_into().expect("parse input field");
            impls.load_input_by_field(buf, a2 as usize, a3 as usize, source, field)
        }),
        consts::SYS_LOAD_SCRIPT => load_to_impls(a0, a1, |buf| impls.load_script(buf, a2 as usize)),
        consts::SYS_LOAD_SCRIPT_HASH => {
            load_to_impls(a0, a1, |buf| impls.load_script_hash(buf, a2 as usize))
        }
        consts::SYS_LOAD_TRANSACTION => {
            load_to_impls(a0, a1, |buf| impls.load_transaction(buf, a2 as usize))
        }
        consts::SYS_LOAD_TX_HASH => {
            load_to_impls(a0, a1, |buf| impls.load_tx_hash(buf, a2 as usize))
        }
        consts::SYS_LOAD_WITNESS => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_witness(buf, a2 as usize, a3 as usize, source)
        }),
        consts::SYS_VM_VERSION => impls.vm_version(),
        consts::SYS_CURRENT_CYCLES => impls.current_cycles(),
        consts::SYS_EXEC => {
            let source: Source = a1.try_into().expect("parse source");
            let argv = build_argv(a4, a5 as *const *const i8);
            match impls.exec(a0 as usize, source, a2 as usize, a3 as usize, &argv) {
                Ok(()) => 0,
                Err(e) => e.into(),
            }
        }
        consts::SYS_SPAWN => {
            let source: Source = a1.try_into().expect("parse source");
            let spgs_addr = a4 as *mut SpawnArgs;
            let spgs: &mut SpawnArgs = unsafe { &mut *spgs_addr };

            let argv = build_argv(spgs.argc, spgs.argv);
            let mut fds = Vec::new();
            let mut fd_ptr: *const u64 = spgs.inherited_fds;
            loop {
                let fd = unsafe { fd_ptr.read() };
                if fd == 0 {
                    break;
                }
                fds.push(fd);
                fd_ptr = unsafe { fd_ptr.offset(1) };
            }
            match impls.spawn(a0 as usize, source, a2 as usize, a3 as usize, &argv, &fds) {
                Ok(process_id) => {
                    unsafe { spgs.process_id.write(process_id) };
                    0
                }
                Err(e) => e.into(),
            }
        }
        consts::SYS_PIPE => {
            let fds = a0 as *mut u64;
            match impls.pipe() {
                Ok((fd1, fd2)) => {
                    unsafe { fds.write(fd1) };
                    unsafe { fds.offset(1).write(fd2) };
                    0
                }
                Err(e) => e.into(),
            }
        }
        consts::SYS_INHERITED_FDS => {
            let fds_ptr = a0 as *mut u64;
            let length_ptr = a1 as *mut u64;
            let length = unsafe { length_ptr.read() } as usize;
            let fds = unsafe { core::slice::from_raw_parts_mut(fds_ptr, length) };

            match impls.inherited_fds(fds) {
                Ok(actual_length) => {
                    unsafe { length_ptr.write(actual_length as u64) };
                    0
                }
                Err(e) => e.into(),
            }
        }
        consts::SYS_READ => {
            let buffer_ptr = a1 as *mut u8;
            let length_ptr = a2 as *mut u64;
            let length = unsafe { length_ptr.read() } as usize;
            let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, length) };

            match impls.read(a0, buffer) {
                Ok(read) => {
                    unsafe { length_ptr.write(read as u64) };
                    0
                }
                Err(e) => e.into(),
            }
        }
        consts::SYS_WRITE => {
            let buffer_ptr = a1 as *const u8;
            let length_ptr = a2 as *mut u64;
            let length = unsafe { length_ptr.read() } as usize;
            let buffer = unsafe { core::slice::from_raw_parts(buffer_ptr, length) };

            match impls.write(a0, buffer) {
                Ok(read) => {
                    unsafe { length_ptr.write(read as u64) };
                    0
                }
                Err(e) => e.into(),
            }
        }
        consts::SYS_CLOSE => match impls.close(a0) {
            Ok(()) => 0,
            Err(e) => e.into(),
        },
        consts::SYS_WAIT => match impls.wait(a0) {
            Ok(exit_code) => {
                let p = a1 as *mut i8;
                unsafe { p.write(exit_code) };
                0
            }
            Err(e) => e.into(),
        },
        consts::SYS_PROCESS_ID => impls.process_id(),
        consts::SYS_LOAD_BLOCK_EXTENSION => load_to_impls(a0, a1, |buf| {
            let source: Source = a4.try_into().expect("parse source");
            impls.load_block_extension(buf, a2 as usize, a3 as usize, source)
        }),
        _ => panic!("Unknown syscall: {}", n),
    }
}

fn build_argv<'a>(argc: u64, argv_ptr: *const *const i8) -> Vec<&'a CStr> {
    let mut argv = Vec::with_capacity(argc as usize);
    for i in 0..argc as isize {
        let p: *const i8 = unsafe { argv_ptr.offset(i as isize).read() };
        argv.push(unsafe { CStr::from_ptr(p as *const _) });
    }
    argv
}

fn load_to_impls<F>(a0: u64, a1: u64, f: F) -> u64
where
    F: Fn(&mut [u8]) -> IoResult,
{
    let buf_ptr = a0 as *mut u8;
    let length_ptr = a1 as *mut u64;

    let length = unsafe { length_ptr.read() };
    let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr, length as usize) };

    match f(buf) {
        IoResult::FullyLoaded(loaded) => {
            unsafe { length_ptr.write(loaded as u64) };
            0
        }
        IoResult::PartialLoaded { available, .. } => {
            unsafe { length_ptr.write(available as u64) };
            0
        }
        IoResult::Error(e) => e.into(),
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

/// MemoryPageResult captures response from load cell code.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemoryPageResult {
    pub page_start: u64,
    pub data: [u8; 4096],
    pub flag: u8,
}
