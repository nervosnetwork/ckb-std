pub const SYS_EXIT: u64 = 93;
pub const SYS_VM_VERSION: u64 = 2041;
pub const SYS_CURRENT_CYCLES: u64 = 2042;
pub const SYS_EXEC: u64 = 2043;
pub const SYS_LOAD_TRANSACTION: u64 = 2051;
pub const SYS_LOAD_SCRIPT: u64 = 2052;
pub const SYS_LOAD_TX_HASH: u64 = 2061;
pub const SYS_LOAD_SCRIPT_HASH: u64 = 2062;
pub const SYS_LOAD_CELL: u64 = 2071;
pub const SYS_LOAD_HEADER: u64 = 2072;
pub const SYS_LOAD_INPUT: u64 = 2073;
pub const SYS_LOAD_WITNESS: u64 = 2074;
pub const SYS_LOAD_CELL_BY_FIELD: u64 = 2081;
pub const SYS_LOAD_HEADER_BY_FIELD: u64 = 2082;
pub const SYS_LOAD_INPUT_BY_FIELD: u64 = 2083;
pub const SYS_LOAD_CELL_DATA_AS_CODE: u64 = 2091;
pub const SYS_LOAD_CELL_DATA: u64 = 2092;
pub const SYS_DEBUG: u64 = 2177;
#[cfg(feature = "ckb2023")]
pub const SYS_SPAWN: u64 = 2601;
#[cfg(feature = "ckb2023")]
pub const SYS_WAIT: u64 = 2602;
#[cfg(feature = "ckb2023")]
pub const SYS_PROCESS_ID: u64 = 2603;
#[cfg(feature = "ckb2023")]
pub const SYS_PIPE: u64 = 2604;
#[cfg(feature = "ckb2023")]
pub const SYS_WRITE: u64 = 2605;
#[cfg(feature = "ckb2023")]
pub const SYS_READ: u64 = 2606;
#[cfg(feature = "ckb2023")]
pub const SYS_INHERITED_FD: u64 = 2607;
#[cfg(feature = "ckb2023")]
pub const SYS_CLOSE: u64 = 2608;
#[cfg(feature = "ckb2023")]
pub const SYS_LOAD_BLOCK_EXTENSION: u64 = 2104;

pub const CKB_SUCCESS: u64 = 0;
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
pub enum Source {
    Input = 1,
    Output = 2,
    CellDep = 3,
    HeaderDep = 4,
    GroupInput = 0x0100000000000001,
    GroupOutput = 0x0100000000000002,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
pub enum CellField {
    Capacity = 0,
    DataHash = 1,
    Lock = 2,
    LockHash = 3,
    Type = 4,
    TypeHash = 5,
    OccupiedCapacity = 6,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
pub enum HeaderField {
    EpochNumber = 0,
    EpochStartBlockNumber = 1,
    EpochLength = 2,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
pub enum InputField {
    OutPoint = 0,
    Since = 1,
}
