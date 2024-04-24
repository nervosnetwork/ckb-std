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

    /// Failed to wait. Its value is 5.
    #[cfg(feature = "ckb2023")]
    WaitFailure,
    /// Invalid file descriptor. Its value is 6.
    #[cfg(feature = "ckb2023")]
    InvalidFd,
    /// Reading from or writing to file descriptor failed due to other end closed. Its value is 7.
    #[cfg(feature = "ckb2023")]
    OtherEndClosed,
    /// Max vms has been spawned. Its value is 8.
    #[cfg(feature = "ckb2023")]
    MaxVmsSpawned,
    /// Max fds has been spawned. Its value is 9.
    #[cfg(feature = "ckb2023")]
    MaxFdsCreated,

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
