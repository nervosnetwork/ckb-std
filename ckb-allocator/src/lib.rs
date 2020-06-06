#![cfg_attr(not(test), no_std)]

pub mod block_list_alloc;
pub mod mixed_alloc;

/// reexports
pub use buddy_alloc;

#[cfg(test)]
mod tests;
