#![cfg_attr(not(test), no_std)]

pub mod bitmap_alloc;
pub mod mixed_alloc;

/// reexports
pub use buddy_alloc;

#[cfg(test)]
mod tests;
