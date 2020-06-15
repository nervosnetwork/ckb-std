/// Defines global allocator
///
///
/// # Example
///
/// ```
/// // define global allocator
/// default_alloc!()
///
/// // Default allocator uses a mixed allocation strategy:
/// //
/// // * Fixed block heap, only allocate fixed size(64B) memory block
/// // * Dynamic memory heap, allocate any size memory block
/// //
/// // User can invoke macro with arguments to customize the heap size
/// // The default heap size arguments are:
/// // (fixed heap size 4KB, dynamic heap size 516KB, dynamic heap min memory block 64B)
/// default_alloc!(4 * 1024, 516 * 1024, 64)
/// ```
#[macro_export]
macro_rules! default_alloc {
    () => {
        default_alloc!(4 * 1024, 516 * 1024, 64);
    };
    ($fixed_block_heap_size:expr, $heap_size:expr, $min_block_size:expr) => {
        static mut _BUDDY_HEAP: [u8; $heap_size] = [0u8; $heap_size];
        static mut _FIXED_BLOCK_HEAP: [u8; $fixed_block_heap_size] = [0u8; $fixed_block_heap_size];

        #[global_allocator]
        static ALLOC: $crate::ckb_allocator::mixed_alloc::MixedAlloc = unsafe {
            let block_list_alloc = $crate::ckb_allocator::block_list_alloc::BlockListAlloc::new(
                _FIXED_BLOCK_HEAP.as_ptr(),
                $fixed_block_heap_size,
            );
            let buddy_alloc = $crate::ckb_allocator::buddy_alloc::NonThreadsafeAlloc::new(
                _BUDDY_HEAP.as_ptr(),
                $heap_size,
                $min_block_size,
            );
            $crate::ckb_allocator::mixed_alloc::MixedAlloc::new(block_list_alloc, buddy_alloc)
        };
    };
}
