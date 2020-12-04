// re-export to maintain compatible with old versions
pub use crate::error::SysError;

#[cfg(not(feature = "simulator"))]
mod native;
#[cfg(not(feature = "simulator"))]
pub use native::*;

#[cfg(feature = "simulator")]
mod simulator;
#[cfg(feature = "simulator")]
pub use simulator::*;
