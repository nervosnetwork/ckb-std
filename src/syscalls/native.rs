use crate::{ckb_constants::*, error::SysError};
#[cfg(target_arch = "riscv64")]
use core::arch::asm;
use core::ffi::CStr;

#[cfg(target_arch = "riscv64")]
unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
      "ecall",
      inout("a0") a0,
      in("a1") a1,
      in("a2") a2,
      in("a3") a3,
      in("a4") a4,
      in("a5") a5,
      in("a6") a6,
      in("a7") a7
    );
    a0
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn syscall(
    _a0: u64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
    _a7: u64,
) -> u64 {
    u64::MAX
}

/// Exit, this script will be terminated after the exit syscall.
/// exit code `0` represents verification is success, others represent error code.
pub fn exit(code: i8) -> ! {
    unsafe { syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}

/// Load data
/// Return data length or syscall error
fn syscall_load(
    buf_ptr: *mut u8,
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

/// Load transaction hash
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
///
/// # Example
///
/// ```
/// let mut tx_hash = [0u8; 32];
/// let len = load_tx_hash(&mut tx_hash, 0).unwrap();
/// assert_eq!(len, tx_hash.len());
/// ```
pub fn load_tx_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        0,
        0,
        0,
        0,
        SYS_LOAD_TX_HASH,
    )
}

/// Load script hash
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
///
/// # Example
///
/// ```
/// let mut script_hash = [0u8; 32];
/// let len = load_script_hash(&mut script_hash, 0).unwrap();
/// assert_eq!(len, script_hash.len());
/// ```
pub fn load_script_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        0,
        0,
        0,
        0,
        SYS_LOAD_SCRIPT_HASH,
    )
}

/// Load cell
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell
/// * `source` - source of cell
pub fn load_cell(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL,
    )
}

/// Load input
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell
/// * `source` - source of cell
pub fn load_input(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_INPUT,
    )
}

/// Load header
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell or header
/// * `source` - source
pub fn load_header(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_HEADER,
    )
}

/// Load witness
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell
/// * `source` - source
pub fn load_witness(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_WITNESS,
    )
}

/// Load transaction
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
pub fn load_transaction(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        0,
        0,
        0,
        0,
        SYS_LOAD_TRANSACTION,
    )
}

/// Load cell by field
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell
/// * `source` - source of cell
/// * `field` - field of cell
///
/// # Example
///
/// ```
/// let mut buf = [0u8; size_of::<u64>()];
/// let len = load_cell_by_field(&mut buf, 0, 0, Source::GroupInput, CellField::Capacity).unwrap();
/// assert_eq!(len, buf.len());
/// ```
pub fn load_cell_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_CELL_BY_FIELD,
    )
}

/// Load header by field
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index
/// * `source` - source
/// * `field` - field
///
/// # Example
///
/// ```
/// let mut buf = [0u8; 8];
/// let len = load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochNumber)?;
/// debug_assert_eq!(len, buf.len());
/// ```
pub fn load_header_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_HEADER_BY_FIELD,
    )
}

/// Load input by field
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index
/// * `source` - source
/// * `field` - field
///
/// # Example
///
/// ```
/// let mut buf = [0u8; 8];
/// let len = load_input_by_field(&mut buf, 0, index, source, InputField::Since)?;
/// debug_assert_eq!(len, buf.len());
/// ```
pub fn load_input_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        field as u64,
        0,
        SYS_LOAD_INPUT_BY_FIELD,
    )
}

/// Load cell data, read cell data
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index
/// * `source` - source
pub fn load_cell_data(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL_DATA,
    )
}

/// Load script
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
pub fn load_script(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        0,
        0,
        0,
        0,
        SYS_LOAD_SCRIPT,
    )
}

/// Output debug message
///
/// You should use the macro version syscall: `debug!`
///
/// # Arguments
///
/// * `s` - string to output
pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    unsafe {
        syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_DEBUG);
    }
}

/// Load cell data, read cell data
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf_ptr` - a writable pointer used to receive the data
/// * `len` - length that the `buf_ptr` can receives.
/// * `offset` - offset
/// * `index` - index
/// * `source` - source
pub fn load_cell_data_raw(
    buf_ptr: *mut u8,
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf_ptr,
        len,
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_CELL_DATA,
    )
}

/// Load cell code, read cell data as executable code
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf_ptr` - a writable pointer used to receive the data
/// * `len` - length that the `buf_ptr` can receives.
/// * `content_offset` - offset
/// * `content_size` - read length
/// * `index` - index
/// * `source` - source
pub fn load_cell_code(
    buf_ptr: *mut u8,
    len: usize,
    content_offset: usize,
    content_size: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let ret = unsafe {
        syscall(
            buf_ptr as u64,
            len as u64,
            content_offset as u64,
            content_size as u64,
            index as u64,
            source as u64,
            0,
            SYS_LOAD_CELL_DATA_AS_CODE,
        )
    };
    SysError::build_syscall_result(ret, len, len)
}

/// *VM version* syscall returns current running VM version, so far 2 values will be returned:
///   - Error for Lina CKB-VM version
///   - 1 for the new hardfork CKB-VM version.
///
/// This syscall consumes 500 cycles.
pub fn vm_version() -> Result<u64, SysError> {
    let ret = unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_VM_VERSION) };
    match ret {
        1 | 2 => Ok(ret),
        _ => Err(SysError::Unknown(ret)),
    }
}

/// *Current Cycles* returns current cycle consumption just before executing this syscall.
///  This syscall consumes 500 cycles.
pub fn current_cycles() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_CURRENT_CYCLES) }
}

/// Exec runs an executable file from specified cell data in the context of an
/// already existing machine, replacing the previous executable. The used cycles
/// does not change, but the code, registers and memory of the vm are replaced
/// by those of the new program. It's cycles consumption consists of two parts:
///
/// - Fixed 500 cycles
/// - Initial Loading Cycles (<https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0014-vm-cycle-limits/0014-vm-
///   cycle-limits.md>)
///
/// The arguments used here are:
///
///   * `index`: an index value denoting the index of entries to read.
///   * `source`: a flag denoting the source of cells or witnesses to locate, possible values include:
///       + 1: input cells.
///       + `0x0100000000000001`: input cells with the same running script as current script
///       + 2: output cells.
///       + `0x0100000000000002`: output cells with the same running script as current script
///       + 3: dep cells.
///   * `place`: A value of 0 or 1:
///       + 0: read from cell data
///       + 1: read from witness
///   * `bounds`: high 32 bits means `offset`, low 32 bits means `length`. if `length` equals to zero, it read to end
///               instead of reading 0 bytes.
///   * `argv`: argv is a one-dimensional array of strings
pub fn exec(index: usize, source: Source, place: usize, bounds: usize, argv: &[&CStr]) -> u64 {
    // https://www.gnu.org/software/libc/manual/html_node/Program-Arguments.html
    let argc = argv.len();
    // On some platforms, CStr may be u8, adding as *const i8 is used to reduce such warnings
    let argv_ptr: alloc::vec::Vec<*const i8> =
        argv.iter().map(|e| e.as_ptr() as *const i8).collect();
    unsafe {
        syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds as u64,
            argc as u64,
            argv_ptr.as_ptr() as u64,
            0,
            SYS_EXEC,
        )
    }
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

/// The parent process calls the Spawn system call, which creates a new process (a child process) that is an
/// independent ckb-vm instance. It's important to note that the parent process will not be blocked by the child
/// process as a result of this syscall.
/// Note: available after ckb 2nd hardfork.
///
/// # Arguments
///
/// * `index`, `source`, `bounds` and `place` - same as exec.
/// * `spgs` - spawn arguments.
///
/// Returns success or a syscall error.
///
/// # Scheduler Algorithm V1
///
/// This document describes the design and functionality of a scheduler algorithm, covering process states, system call
/// behavior, message handling, and priority rules. The scheduler manages virtual processes, transitioning them through
/// various states based on operations and interactions, to ensure efficient scheduling.
///
/// # Thread States
///
/// Each process within this scheduler has one of the following six states:
/// * Runnable: The process is ready to execute.
/// * Running: The process is running.
/// * Terminated: The process has completed its execution.
/// * WaitForRead: The process is waiting for data to be available for it to read.
/// * WaitForWrite: The process is waiting for another process to read data it wants to write.
/// * WaitForExit: The process is waiting for another process to exit before it can continue.
///
/// # System Calls and State Transitions
///
/// Specific system calls are responsible for changing the state of a process within this scheduler:
/// * spawn: Creates a new process, initializing it in the Runnable state.
/// * read: Attempts to read data from a file descriptor. If data is unavailable, the process state changes to
///     WaitForRead.
/// * write: Attempts to write data to a file descriptor. If the operation is blocked due to data needing to be read by
///     another process, the process enters the WaitForWrite state.
/// * wait: Waits for a target process to exit. Once the target process has terminated, the waiting process transitions
///     to Runnable.
///
/// # IO Handling and State Recovery
///
/// IO handling allows processes in certain states to transition back to Runnable when specific conditions are met:
/// * A WaitForRead process becomes Runnable once the needed data has been read successfully.
/// * A WaitForWrite process transitions to Runnable once its data has been successfully read by another process.
///
/// # Process Priority
///
/// The scheduler assigns incremental IDs to processes, establishing an execution order:
/// * The root process has an ID of 0.
/// * When multiple processes are in the Runnable state, the scheduler selects the process with the lowest ID to execute
///     first. This ensures a predictable and fair ordering of execution for processes ready to run. The selected
///     process status is changed to Running, and it continues to run until its status is changed.
pub fn spawn(
    index: usize,
    source: Source,
    place: usize,
    bounds: usize,
    spgs: &mut SpawnArgs,
) -> Result<u64, SysError> {
    let ret = unsafe {
        syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds as u64,
            spgs as *mut SpawnArgs as u64,
            0,
            0,
            SYS_SPAWN,
        )
    };
    match ret {
        0 => Ok(unsafe { *spgs.process_id }),
        1 => Err(SysError::IndexOutOfBound),
        2 => Err(SysError::ItemMissing),
        3 => Err(SysError::Encoding),
        6 => Err(SysError::InvalidFd),
        8 => Err(SysError::MaxVmsSpawned),
        x => Err(SysError::Unknown(x)),
    }
}

/// The syscall pauses until the execution of a process specified by pid has ended.
/// Note: available after ckb 2nd hardfork.
///
/// # Arguments
///
/// * `pid` - process id
///
/// Returns exit code.
pub fn wait(pid: u64) -> Result<i8, SysError> {
    let mut code: u64 = 0;
    let ret = unsafe { syscall(pid, &mut code as *mut u64 as u64, 0, 0, 0, 0, 0, SYS_WAIT) };
    match ret {
        0 => Ok(code as i8),
        5 => Err(SysError::WaitFailure),
        x => Err(SysError::Unknown(x)),
    }
}

/// This syscall is used to get the current process id. Root process ID is 0.
/// Note: available after ckb 2nd hardfork.
pub fn process_id() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_PROCESS_ID) }
}

/// This syscall create a pipe with read-write pair of file descriptions. The file descriptor with read permission is
/// located at fds[0], and the corresponding file descriptor with write permission is located at fds[1].
/// Note: available after ckb 2nd hardfork.
pub fn pipe() -> Result<(u64, u64), SysError> {
    let mut fds: [u64; 2] = [0, 0];
    let ret = unsafe { syscall(fds.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_PIPE) };
    match ret {
        0 => Ok((fds[0], fds[1])),
        9 => Err(SysError::MaxFdsCreated),
        x => Err(SysError::Unknown(x)),
    }
}

/// This syscall reads data from a pipe via a file descriptor. The syscall Read attempts to read up to value pointed by
/// length bytes from file descriptor fd into the buffer, and the actual length of data read is returned.
/// Note: available after ckb 2nd hardfork.
pub fn read(fd: u64, buffer: &mut [u8]) -> Result<usize, SysError> {
    let mut l: u64 = buffer.len() as u64;
    let ret = unsafe {
        syscall(
            fd,
            buffer.as_mut_ptr() as u64,
            &mut l as *mut u64 as u64,
            0,
            0,
            0,
            0,
            SYS_READ,
        )
    };
    match ret {
        0 => Ok(l as usize),
        1 => Err(SysError::IndexOutOfBound),
        6 => Err(SysError::InvalidFd),
        7 => Err(SysError::OtherEndClosed),
        x => Err(SysError::Unknown(x)),
    }
}

/// This syscall writes data to a pipe via a file descriptor. The syscall Write writes up to value pointed by length
/// bytes from the buffer, and the actual length of data written is returned.
/// Note: available after ckb 2nd hardfork.
pub fn write(fd: u64, buffer: &[u8]) -> Result<usize, SysError> {
    let mut l: u64 = buffer.len() as u64;
    let ret = unsafe {
        syscall(
            fd,
            buffer.as_ptr() as u64,
            &mut l as *mut u64 as u64,
            0,
            0,
            0,
            0,
            SYS_WRITE,
        )
    };
    match ret {
        0 => Ok(l as usize),
        1 => Err(SysError::IndexOutOfBound),
        6 => Err(SysError::InvalidFd),
        7 => Err(SysError::OtherEndClosed),
        x => Err(SysError::Unknown(x)),
    }
}

/// This syscall retrieves the file descriptors available to the current process, which are passed in from the parent
/// process. These results are copied from the inherited_fds parameter of the Spawn syscall.
/// Note: available after ckb 2nd hardfork.
pub fn inherited_fds(fds: &mut [u64]) -> u64 {
    let mut l: u64 = fds.len() as u64;
    unsafe {
        syscall(
            fds.as_mut_ptr() as u64,
            &mut l as *mut u64 as u64,
            0,
            0,
            0,
            0,
            0,
            SYS_INHERITED_FDS,
        )
    };
    l
}

/// This syscall manually closes a file descriptor. After calling this, any attempt to read/write the file descriptor
/// pointed to the other end would fail.
/// Note: available after ckb 2nd hardfork.
pub fn close(fd: u64) -> Result<(), SysError> {
    let ret = unsafe { syscall(fd, 0, 0, 0, 0, 0, 0, SYS_CLOSE) };
    match ret {
        0 => Ok(()),
        6 => Err(SysError::InvalidFd),
        x => Err(SysError::Unknown(x)),
    }
}

/// Load extension field associated either with an input cell, a dep cell, or
/// a header dep based on source and index value.
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `offset` - offset
/// * `index` - index of cell
/// * `source` - source of cell
///
/// Note: available after ckb 2nd hardfork.
pub fn load_block_extension(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        offset,
        index as u64,
        source as u64,
        0,
        0,
        SYS_LOAD_BLOCK_EXTENSION,
    )
}
