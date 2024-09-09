// re-export to maintain compatible with old versions
pub use crate::error::SysError;

#[cfg(not(feature = "native-simulator"))]
mod native;
#[cfg(not(feature = "native-simulator"))]
pub use native::*;

#[cfg(feature = "native-simulator")]
mod simulator;
#[cfg(feature = "native-simulator")]
pub use simulator::*;
