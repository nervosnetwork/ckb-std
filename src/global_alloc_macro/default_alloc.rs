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
        $crate::default_alloc!({ 4 * 1024 }, { 516 * 1024 }, 64);
    };
    ($fixed_block_heap_size:expr, $heap_size:expr, $min_block_size:expr) => {
        #[repr(align(64))]
        struct _AlignedHeap<const N: usize>([u8; N]);

        static mut _BUDDY_HEAP: _AlignedHeap<$heap_size> = _AlignedHeap([0u8; $heap_size]);
        static mut _FIXED_BLOCK_HEAP: _AlignedHeap<$fixed_block_heap_size> =
            _AlignedHeap([0u8; $fixed_block_heap_size]);

        #[global_allocator]
        static ALLOC: $crate::buddy_alloc::NonThreadsafeAlloc = unsafe {
            let fast_param = $crate::buddy_alloc::FastAllocParam::new(
                _FIXED_BLOCK_HEAP.0.as_ptr(),
                $fixed_block_heap_size,
            );
            let buddy_param = $crate::buddy_alloc::BuddyAllocParam::new_with_zero_filled(
                _BUDDY_HEAP.0.as_ptr(),
                $heap_size,
                $min_block_size,
            );
            $crate::buddy_alloc::NonThreadsafeAlloc::new(fast_param, buddy_param)
        };
    };
}
