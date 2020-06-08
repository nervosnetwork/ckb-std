//! Block list allocator
//! Optimized for fixed small memory block.

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;

// Block size is 64 Bytes
pub const BLOCK_SIZE: usize = 64;

struct Node {
    next: *mut Node,
    prev: *mut Node,
}

impl Node {
    fn init(list: *mut Node) {
        unsafe {
            (*list).next = list;
            (*list).prev = list;
        }
    }

    fn remove(list: *mut Node) {
        unsafe {
            (*(*list).prev).next = (*list).next;
            (*(*list).next).prev = (*list).prev;
        }
    }

    fn pop(list: *mut Node) -> *mut Node {
        let n_list: *mut Node = unsafe { (*list).next };
        Self::remove(n_list);
        n_list
    }

    fn push(list: *mut Node, p: *mut u8) {
        let p = p.cast::<Node>();
        unsafe {
            let n_list: Node = Node {
                prev: list,
                next: (*list).next,
            };
            p.write_unaligned(n_list);
            (*(*list).next).prev = p;
            (*list).next = p;
        }
    }

    fn is_empty(list: *const Node) -> bool {
        unsafe { (*list).next as *const Node == list }
    }
}

pub struct BlockListAllocInner {
    /// memory start addr
    base_addr: usize,
    /// memory end addr
    end_addr: usize,
    free: *mut Node,
}

impl BlockListAllocInner {
    /// # Safety
    ///
    /// The `base_addr..(base_addr + len)` must be allocated before using,
    /// and must guarantee no others write to the memory range, otherwise behavior is undefined.
    pub unsafe fn new(base_addr: *const u8, len: usize) -> Self {
        let base_addr = base_addr as usize;
        let end_addr = base_addr + len;
        debug_assert_eq!(len % BLOCK_SIZE, 0);

        let nblocks = len / BLOCK_SIZE;

        // initialize free list
        let free = base_addr as *mut Node;
        Node::init(free);

        let mut addr = base_addr;
        for _ in 0..(nblocks - 1) {
            addr += BLOCK_SIZE;
            Node::push(free, addr as *mut u8);
        }

        BlockListAllocInner {
            base_addr,
            end_addr,
            free,
        }
    }

    pub fn contains_ptr(&self, p: *mut u8) -> bool {
        let addr = p as usize;
        addr >= self.base_addr && addr < self.end_addr
    }

    pub fn malloc(&mut self, nbytes: usize) -> *mut u8 {
        if nbytes > BLOCK_SIZE || self.free.is_null() {
            return core::ptr::null_mut();
        }

        let is_last = Node::is_empty(self.free);
        let p = Node::pop(self.free) as *mut u8;
        if is_last {
            self.free = core::ptr::null_mut();
        }
        p
    }

    pub fn free(&mut self, p: *mut u8) {
        debug_assert!(self.contains_ptr(p));
        if self.free.is_null() {
            let n = p.cast();
            Node::init(n);
            self.free = n;
        } else {
            Node::push(self.free, p);
        }
    }
}

pub struct BlockListAlloc {
    base_addr: *const u8,
    len: usize,
    inner: RefCell<Option<BlockListAllocInner>>,
}

impl BlockListAlloc {
    pub const unsafe fn new(base_addr: *const u8, len: usize) -> Self {
        BlockListAlloc {
            base_addr,
            len,
            inner: RefCell::new(None),
        }
    }

    unsafe fn with_inner<R, F: FnOnce(&mut BlockListAllocInner) -> R>(&self, f: F) -> R {
        let mut inner = self.inner.borrow_mut();
        if inner.is_none() {
            inner.replace(BlockListAllocInner::new(self.base_addr, self.len));
        }
        f(inner.as_mut().unwrap())
    }

    pub unsafe fn contains_ptr(&self, p: *mut u8) -> bool {
        self.with_inner(|alloc| alloc.contains_ptr(p))
    }
}

unsafe impl GlobalAlloc for BlockListAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.with_inner(|alloc| alloc.malloc(layout.size()))
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.with_inner(|alloc| alloc.free(ptr));
    }
}

unsafe impl Sync for BlockListAlloc {}
