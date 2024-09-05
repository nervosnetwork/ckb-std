#![cfg_attr(not(feature = "native-simulator"), no_std)]

#[cfg(feature = "native-simulator")]
pub mod entry;
#[cfg(feature = "native-simulator")]
pub mod error;
