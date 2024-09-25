//! Implementation of Type ID
//!
//! This module provides functionality for validating and checking Type IDs in
//! CKB transactions. It requires "type-id" feature in ckb-std enabled.
//!
//! For more details, see the [Type ID
//! RFC](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md#type-id).
//!
//! Note: Type ID cells are allowed to be burned.
//!
use crate::{
    ckb_constants::Source,
    error::SysError,
    high_level::{load_cell_type_hash, load_input, load_script, load_script_hash, QueryIter},
    syscalls::load_cell,
};
use ckb_hash::new_blake2b;
use ckb_types::prelude::Entity;

fn is_cell_present(index: usize, source: Source) -> bool {
    let buf = &mut [];
    matches!(
        load_cell(buf, 0, index, source),
        Ok(_) | Err(SysError::LengthNotEnough(_))
    )
}

fn locate_index() -> Result<usize, SysError> {
    let hash = load_script_hash()?;

    let index = QueryIter::new(load_cell_type_hash, Source::Output)
        .position(|type_hash| type_hash == Some(hash))
        .ok_or(SysError::TypeIDError)?;

    Ok(index)
}

///
/// Validates the Type ID in a flexible manner.
///
/// This function performs a low-level validation of the Type ID. It checks for the
/// presence of cells in the transaction and validates the Type ID based on whether
/// it's a minting operation or a transfer.
///
/// # Arguments
///
/// * `type_id` - A 32-byte array representing the Type ID to validate.
///
/// # Returns
///
/// * `Ok(())` if the Type ID is valid.
/// * `Err(SysError::TypeIDError)` if the validation fails.
///
/// # Note
///
/// For most use cases, it's recommended to use the `check_type_id` function instead,
/// which expects the Type ID to be included in the script `args`.
///
/// # Examples
///
/// ```no_run
/// use ckb_std::type_id::validate_type_id;
///
/// let type_id = [0u8; 32];
/// validate_type_id(type_id)?;
/// ```
pub fn validate_type_id(type_id: [u8; 32]) -> Result<(), SysError> {
    // after this checking, there are 3 cases:
    // 1. 0 input cell and 1 output cell, it's minting operation
    // 2. 1 input cell and 1 output cell, it's transfer operation
    // 3. 1 input cell and 0 output cell, it's burning operation(allowed)
    if is_cell_present(1, Source::GroupInput) || is_cell_present(1, Source::GroupOutput) {
        return Err(SysError::TypeIDError);
    }

    // case 1: minting operation
    if !is_cell_present(0, Source::GroupInput) {
        let index = locate_index()? as u64;
        let input = load_input(0, Source::Input)?;
        let mut blake2b = new_blake2b();
        blake2b.update(input.as_slice());
        blake2b.update(&index.to_le_bytes());
        let mut ret = [0; 32];
        blake2b.finalize(&mut ret);

        if ret != type_id {
            return Err(SysError::TypeIDError);
        }
    }
    // case 2 & 3: for the `else` part, it's transfer operation or burning operation
    Ok(())
}

fn load_id_from_args(offset: usize) -> Result<[u8; 32], SysError> {
    let script = load_script()?;
    let args = script.as_reader().args();
    let args_data = args.raw_data();

    args_data
        .get(offset..offset + 32)
        .ok_or(SysError::TypeIDError)?
        .try_into()
        .map_err(|_| SysError::TypeIDError)
}

///
/// Validates that the script follows the Type ID rule.
///
/// This function checks if the Type ID (a 32-byte value) stored in the script's `args`
/// at the specified offset is valid according to the Type ID rules.
///
/// # Arguments
///
/// * `offset` - The byte offset in the script's `args` where the Type ID starts.
///
/// # Returns
///
/// * `Ok(())` if the Type ID is valid.
/// * `Err(SysError::TypeIDError)` if the Type ID is invalid or cannot be retrieved.
///
/// # Examples
///
/// ```no_run
/// use ckb_std::type_id::check_type_id;
///
/// fn main() -> Result<(), ckb_std::error::SysError> {
///     // Check the Type ID stored at the beginning of the script args
///     check_type_id(0)?;
///     Ok(())
/// }
/// ```
///
/// # Note
///
/// This function internally calls `load_id_from_args` to retrieve the Type ID
/// and then `validate_type_id` to perform the actual validation.
pub fn check_type_id(offset: usize) -> Result<(), SysError> {
    let type_id = load_id_from_args(offset)?;
    validate_type_id(type_id)?;
    Ok(())
}
