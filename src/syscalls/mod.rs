// re-export to maintain compatible with old versions
pub use crate::error::SysError;

mod internal;
pub mod traits;

#[cfg(not(any(feature = "native-simulator", feature = "stub-syscalls")))]
mod native;
#[cfg(not(any(feature = "native-simulator", feature = "stub-syscalls")))]
pub use native::*;

#[cfg(feature = "native-simulator")]
mod simulator;
#[cfg(feature = "native-simulator")]
pub use simulator::*;

#[cfg(feature = "stub-syscalls")]
mod stub;
#[cfg(feature = "stub-syscalls")]
pub use stub::*;
