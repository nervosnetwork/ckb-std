/// Syscall errors
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SysError {
    /// Index out of bound
    ///
    /// `index` is past the last item of `source`, or `source` does not apply to the syscall,
    /// such as loading an input or a header from `Source::Output`.
    /// [`QueryIter`](crate::high_level::QueryIter) treats it as the end of iteration,
    /// so iterating over the wrong source silently yields nothing.
    IndexOutOfBound,
    /// Field is missing for the target
    ///
    /// The item exists but lacks the requested value, for example the type script of a cell
    /// without one, or the header of an input or dep cell whose block hash is not in `header_deps`.
    ItemMissing,
    /// The data did not fit in the buffer; contains the data length from `offset`
    ///
    /// This is not a failed load: the buffer is already filled with the first bytes of
    /// the data from `offset`, and the value is the full length of that data.
    /// For example, with 100 bytes of data, a 32-byte buffer and `offset` 0,
    /// the buffer holds bytes `0..32` and the error is `LengthNotEnough(100)`;
    /// calling again with `offset` 32 loads bytes `32..64` and returns `LengthNotEnough(68)`.
    ///
    /// When only a prefix is needed, treat this error as success.
    /// See [Partial Loading](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md#partial-loading).
    LengthNotEnough(usize),
    /// Data encoding error
    ///
    /// Loaded bytes do not parse as the expected Molecule type in the high-level loaders,
    /// or as hex in `decode_hex`; `spawn` also returns it for an out-of-bound slice.
    Encoding,

    /// Failed to wait. Its value is 5.
    WaitFailure,
    /// Invalid file descriptor. Its value is 6.
    InvalidFd,
    /// Reading from or writing to file descriptor failed due to other end closed. Its value is 7.
    OtherEndClosed,
    /// Max vms has been spawned. Its value is 8.
    MaxVmsSpawned,
    /// Max fds has been spawned. Its value is 9.
    MaxFdsCreated,
    /// Type ID Error
    #[cfg(feature = "type-id")]
    TypeIDError,
    /// Unknown syscall error number
    Unknown(u64),
}

impl SysError {
    #[allow(dead_code)]
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
