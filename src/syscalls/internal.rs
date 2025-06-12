#[repr(C)]
pub struct SpawnArgs {
    /// argc contains the number of arguments passed to the program.
    pub argc: u64,
    /// argv is a one-dimensional array of strings.
    pub argv: *const *const i8,
    /// a pointer used to save the process_id of the child process.
    pub process_id: *mut u64,
    /// an array representing the file descriptors passed to the child process. It must end with zero.
    pub inherited_fds: *const u64,
}

#[cfg(target_arch = "riscv64")]
pub unsafe fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a7: u64) -> u64 {
    unsafe {
        core::arch::asm!(
          "ecall",
          inout("a0") a0,
          in("a1") a1,
          in("a2") a2,
          in("a3") a3,
          in("a4") a4,
          in("a5") a5,
          in("a7") a7
        );
        a0
    }
}

#[cfg(not(target_arch = "riscv64"))]
pub unsafe fn syscall(_a0: u64, _a1: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a7: u64) -> u64 {
    u64::MAX
}
