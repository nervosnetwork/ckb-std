#![no_std]
#![no_main]

use alloc::vec;
use ckb_std;

ckb_std::default_alloc!();
ckb_std::entry!(program_entry);

fn program_entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

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

impl From<ckb_std::error::SysError> for Error {
    fn from(err: ckb_std::error::SysError) -> Self {
        match err {
            ckb_std::error::SysError::IndexOutOfBound => Self::IndexOutOfBound,
            ckb_std::error::SysError::ItemMissing => Self::ItemMissing,
            ckb_std::error::SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            ckb_std::error::SysError::Encoding => Self::Encoding,
            ckb_std::error::SysError::WaitFailure => Self::WaitFailure,
            ckb_std::error::SysError::InvalidFd => Self::InvalidFd,
            ckb_std::error::SysError::OtherEndClosed => Self::OtherEndClosed,
            ckb_std::error::SysError::MaxVmsSpawned => Self::MaxVmsSpawned,
            ckb_std::error::SysError::MaxFdsCreated => Self::MaxFdsCreated,
            ckb_std::error::SysError::Unknown(err_code) => {
                panic!("unexpected sys error {}", err_code)
            }
        }
    }
}

pub fn main() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    let mut std_fds: [u64; 2] = [0; 2];
    ckb_std::syscalls::inherited_fds(&mut std_fds);
    let mut out = vec![];
    for arg in argv {
        out.extend_from_slice(arg.to_bytes());
    }
    let len = ckb_std::syscalls::write(std_fds[1], &out)?;
    assert_eq!(len, 10);
    Ok(())
}
