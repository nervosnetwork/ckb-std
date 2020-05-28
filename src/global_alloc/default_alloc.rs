/// Defines global allocator
///
///
/// # Example
///
/// ```
/// // define global allocator
/// default_alloc!()
///
/// // customize the heap size(default heap size is 64KB, min allocated memory is 16B)
/// default_alloc!(64 * 1024, 16)
/// ```
#[macro_export]
macro_rules! default_alloc {
    () => {
        default_alloc!(64 * 1024, 16);
    };
    ($heap_size:expr, $min_block_size:expr) => {
        const _HEAP_SIZE: usize = $heap_size;
        static mut _HEAP: [u8; _HEAP_SIZE] = [0u8; _HEAP_SIZE];

        #[global_allocator]
        static ALLOC: $crate::buddy_alloc::NonThreadsafeAlloc = unsafe {
            $crate::buddy_alloc::NonThreadsafeAlloc::new(
                _HEAP.as_ptr(),
                _HEAP_SIZE,
                $min_block_size,
            )
        };
    };
}
