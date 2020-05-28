/// Defines global allocator
///
/// # Example
///
/// ```
/// // define global allocator
/// libc_alloc!()
///
/// // customize the heap size(default heap size is 64KB)
/// libc_alloc!(64 * 1024)
/// ```
#[macro_export]
macro_rules! libc_alloc {
    () => {
        libc_alloc!(64 * 1024);
    };
    ($heap_size:expr) => {
        extern crate alloc;

        #[no_mangle]
        fn _sbrk(inc: isize) -> *mut u8 {
            const _HEAP_SIZE: usize = $heap_size;
            static mut _HEAP: [u8; _HEAP_SIZE] = [0u8; _HEAP_SIZE];
            static mut _OFFSET: isize = 0;

            unsafe {
                let old_offset = _OFFSET;
                _OFFSET += inc;
                if _OFFSET < 0 || _OFFSET >= _HEAP_SIZE as isize {
                    panic!("heap offset overflow");
                }
                _HEAP.as_mut_ptr().add(old_offset as usize)
            }
        }

        #[global_allocator]
        static ALLOC: $crate::libc_alloc::LibCAlloc = unsafe { $crate::libc_alloc::LibCAlloc {} };
    };
}
