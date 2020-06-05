use crate::bitmap_alloc::{BitmapAlloc, MEMORY_SIZE, MIN_MEMORY_BLOCK};

fn with_allocator<F: FnOnce(BitmapAlloc)>(f: F) {
    f(BitmapAlloc::default());
}

#[test]
fn test_basic_malloc() {
    // alloc a min block
    with_allocator(|mut allocator| {
        let p = allocator.malloc(512);
        let p_addr = p as usize;
        assert!(!p.is_null());
        // memory writeable
        unsafe { p.write(42) };
        assert_eq!(p_addr, p as usize);
        assert_eq!(unsafe { *p }, 42);
    });
}

#[test]
fn test_multiple_malloc() {
    with_allocator(|mut allocator| {
        let mut available_bytes = MEMORY_SIZE;
        // alloc serveral sized blocks
        while available_bytes >= MIN_MEMORY_BLOCK {
            let bytes = MIN_MEMORY_BLOCK;
            assert!(!allocator.malloc(bytes).is_null());
            available_bytes -= bytes;
        }
    });
}

#[test]
fn test_small_size_malloc() {
    with_allocator(|mut allocator| {
        let mut available_bytes = MEMORY_SIZE;
        while available_bytes >= MIN_MEMORY_BLOCK {
            assert!(!allocator.malloc(MIN_MEMORY_BLOCK).is_null());
            available_bytes -= MIN_MEMORY_BLOCK;
        }
        // memory should be drained, we can't allocate even 1 byte
        assert!(allocator.malloc(1).is_null());
    });
}

#[test]
fn test_malloc_maximum() {
    with_allocator(|mut allocator| {
        let p = allocator.malloc(MEMORY_SIZE);
        assert!(!p.is_null());
    });
}

#[test]
fn test_fail_malloc() {
    // not enough memory since we only have HEAP_SIZE bytes,
    // and the allocator itself occupied few bytes
    with_allocator(|mut allocator| {
        let p = allocator.malloc(MEMORY_SIZE + 1);
        assert!(p.is_null());
    });
}

#[test]
fn test_malloc_and_free() {
    fn _test_malloc_and_free(times: usize) {
        with_allocator(|mut allocator| {
            for _i in 0..times {
                let mut available_bytes = MEMORY_SIZE;
                let mut ptrs = Vec::new();
                // alloc serveral sized blocks
                while available_bytes >= MIN_MEMORY_BLOCK {
                    let bytes = MIN_MEMORY_BLOCK;
                    let p = allocator.malloc(bytes);
                    assert!(!p.is_null());
                    ptrs.push((p, bytes));
                    available_bytes -= bytes;
                }
                // space is drained
                assert!(allocator.malloc(1).is_null());
                // free allocated blocks
                for (ptr, bytes) in ptrs {
                    assert!(allocator.contains_ptr(ptr));
                    allocator.free(ptr, bytes);
                }
            }
        });
    }
    _test_malloc_and_free(10);
}

#[test]
fn test_free_bug() {
    with_allocator(|mut allocator| {
        let p1 = allocator.malloc(32);
        allocator.free(p1, 32);
        let p2 = allocator.malloc(4096);
        let p3 = allocator.malloc(138);
        allocator.free(p2, 4096);
        allocator.free(p3, 138);
    });
}
