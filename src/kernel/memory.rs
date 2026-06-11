use core::alloc::{ GlobalAlloc, Layout };
use core::ptr;
use core::sync::atomic::{ AtomicUsize, Ordering };

pub struct BumpAllocator {
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init(&self, heap_start: usize, heap_end: usize) {
        self.heap_start.store(heap_start, Ordering::SeqCst);
        self.heap_end.store(heap_end, Ordering::SeqCst);
        self.next.store(heap_start, Ordering::SeqCst);
    }

    pub fn used_memory(&self) -> usize {
        let start = self.heap_start.load(Ordering::SeqCst);
        let next = self.next.load(Ordering::SeqCst);
        if start == 0 || next < start {
            0
        } else {
            next - start
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        loop {
            let current_next = self.next.load(Ordering::SeqCst);
            if current_next == 0 {
                return ptr::null_mut();
            } // Not initialized

            let aligned_next = (current_next + align - 1) & !(align - 1);
            let next = aligned_next + size;

            if next > self.heap_end.load(Ordering::SeqCst) {
                return ptr::null_mut();
            }

            if
                self.next
                    .compare_exchange(current_next, next, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
            {
                return aligned_next as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not support deallocation
    }
}

pub const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap

#[allow(dead_code)]
#[repr(align(4096))]
pub struct HeapMemory([u8; HEAP_SIZE]);
pub static mut HEAP_MEM: HeapMemory = HeapMemory([0; HEAP_SIZE]);

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();
