/// Syscall errors
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SysError {
    /// Index out of bound
    IndexOutOfBound,
    /// Field is missing for the target
    ItemMissing,
    /// Buffer length is not enough, error contains actual data length
    LengthNotEnough(usize),
    /// Data encoding error
    Encoding,

    #[cfg(feature = "ckb2023")]
    /// Content Length must be less than 256K.
    SpawnExceededMaxContentLength,
    #[cfg(feature = "ckb2023")]
    /// MemoryLimit is between 1 and 8.
    SpawnWrongMemoryLimit,
    #[cfg(feature = "ckb2023")]
    /// There is a maximum call depth limit by peak memory.
    SpawnExceededMaxPeakMemory,

    /// Unknown syscall error number
    Unknown(u64),
}

impl SysError {
    pub(crate) fn build_syscall_result(
        errno: u64,
        load_len: usize,
        actual_data_len: usize,
    ) -> Result<usize, SysError> {
        use SysError::*;

        match errno {
            0 => {
                if actual_data_len > load_len {
                    return Err(LengthNotEnough(actual_data_len));
                }
                Ok(actual_data_len)
            }
            1 => Err(IndexOutOfBound),
            2 => Err(ItemMissing),
            _ => Err(Unknown(errno)),
        }
    }
}
