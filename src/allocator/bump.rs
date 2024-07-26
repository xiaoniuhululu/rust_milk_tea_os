use alloc::alloc::{GlobalAlloc, Layout};
use super::{align_up, Locked};
// use super::{Locked};
use core::ptr;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// 使用给定的堆边界初始化凹凸分配器。 
    /// 该方法不安全，因为调用者必须确保给定的 
    /// 内存范围未被使用。 此外，该方法只能调用一次。
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

// unsafe impl GlobalAlloc for BumpAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         // TODO alignment and bounds check
//         let alloc_start = self.next;
//         self.next = alloc_start + layout.size();
//         self.allocations += 1;
//         alloc_start as *mut u8
//     }

//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//         todo!();
//     }
// }
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock(); // get a mutable reference

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock(); // get a mutable reference

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}


// Align the given address `addr` upwards to alignment `align`.
// fn align_up(addr: usize, align: usize) -> usize {
//     let remainder = addr % align;
//     if remainder == 0 {
//         addr // addr already aligned
//     } else {
//         addr - remainder + align
//     }
// }