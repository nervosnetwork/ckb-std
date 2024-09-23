use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    WaitFailure,
    InvalidFd,
    OtherEndClosed,
    MaxVmsSpawned,
    MaxFdsCreated,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            WaitFailure => Self::WaitFailure,
            InvalidFd => Self::InvalidFd,
            OtherEndClosed => Self::OtherEndClosed,
            MaxVmsSpawned => Self::MaxVmsSpawned,
            MaxFdsCreated => Self::MaxFdsCreated,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            _ => panic!("other sys error"),
        }
    }
}
