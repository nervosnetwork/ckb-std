//! Bitmap allocator

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;

/// Min allocated memory is 256B
pub const MIN_MEMORY_BLOCK: usize = 256;
/// Memory size is 64K
pub const MEMORY_SIZE: usize = 1 << 16;
pub const MAX_BLOCKS: usize = 32 * 8;

/// Fix memory 32KB
pub struct BitmapAlloc {
    bitmap: [u8; MAX_BLOCKS / 8],
    memory: [u8; MEMORY_SIZE],
    cursor: usize,
}

const fn roundup_blocks(n: usize) -> usize {
    // MIN_MEMORY_BLOCK
    ((n - 1) >> 8) + 1
}

impl BitmapAlloc {
    pub const fn default() -> Self {
        BitmapAlloc {
            bitmap: [0u8; MAX_BLOCKS / 8],
            memory: [0u8; MEMORY_SIZE],
            cursor: 0,
        }
    }

    fn search_continues(&self, nblocks: usize, start: usize, end: usize) -> Option<usize> {
        if end - start < nblocks {
            return None;
        }

        for i in start..=(end - nblocks) {
            if (i..i + nblocks).all(|i| !self.bit_isset(i)) {
                return Some(i);
            }
        }
        None
    }

    fn bit_isset(&self, i: usize) -> bool {
        let b = self.bitmap[i / 8];
        let m = 1 << (i % 8);
        b & m == m
    }

    fn bit_set(&mut self, i: usize) {
        let b = self.bitmap[i / 8];
        let m = 1 << (i % 8);
        self.bitmap[i / 8] = b | m;
    }

    fn bit_clear(&mut self, i: usize) {
        let b = self.bitmap[i / 8];
        let m = 1 << (i % 8);
        self.bitmap[i / 8] = b & !m;
    }

    pub fn contains_ptr(&self, p: *mut u8) -> bool {
        let ma = self.memory.as_ptr() as usize;
        let pa = p as usize;
        pa >= ma && pa < ma + self.memory.len()
    }

    pub fn malloc(&mut self, nbytes: usize) -> *mut u8 {
        let nblocks = roundup_blocks(nbytes);
        match self
            .search_continues(nblocks, self.cursor, MAX_BLOCKS)
            .or_else(|| {
                if self.cursor != 0 {
                    self.search_continues(nblocks, 0, self.cursor)
                } else {
                    None
                }
            }) {
            Some(n) => {
                for i in n..n + nblocks {
                    self.bit_set(i);
                }
                let new_cursor = nblocks + n;
                if new_cursor == MAX_BLOCKS {
                    self.cursor = 0;
                } else {
                    self.cursor = new_cursor;
                }
                unsafe { self.memory.as_mut_ptr().add(n * MIN_MEMORY_BLOCK) }
            }
            None => core::ptr::null_mut(),
        }
    }

    pub fn free(&mut self, p: *mut u8, nbytes: usize) {
        let n = roundup_blocks(nbytes);
        let start = (p as usize - self.memory.as_ptr() as usize) / MIN_MEMORY_BLOCK;
        for i in start..start + n {
            self.bit_clear(i);
        }
    }
}

/// A non thread safe global allocator
pub struct GlobalBitmapAlloc(RefCell<BitmapAlloc>);

impl GlobalBitmapAlloc {
    pub const fn new() -> Self {
        GlobalBitmapAlloc(RefCell::new(BitmapAlloc::default()))
    }
}

unsafe impl GlobalAlloc for GlobalBitmapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.borrow_mut().malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.borrow_mut().free(ptr, layout.size())
    }
}

unsafe impl Sync for GlobalBitmapAlloc {}
