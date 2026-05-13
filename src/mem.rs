//! Memory heap allocation for DOS programs.
//! Uses conventional memory for DOS programs, from the EXE stack end to extended BIOS data area (EBDA)
//! Uses linear algorithm for allocating memory, which is not optimal, but it's simple and works.

use core::{
    alloc::{GlobalAlloc, Layout},
    mem::size_of,
    ptr::null_mut,
};

use crate::dos::get_data_seg;

const LAST_MEMORY_BYTE_ADDR: usize = 0x9FBFF; // (0X9000 << 4) + 0xFBFF, last byte of memory before extended BIOS data area
const ALLOCATOR_BLOCK_SIZE: usize = size_of::<AllocatorBlock>();
const MIN_BLOCK_USEFUL_SIZE: usize = 16;
const ALLOCATOR_ALIGN: usize = 16;

unsafe extern "C" {
    static _heap: u8;
}

#[repr(align(16))]
pub struct AllocatorBlock {
    next: Option<*mut AllocatorBlock>,
    prev: Option<*mut AllocatorBlock>,
    used: bool,
}

impl AllocatorBlock {
    fn ptr(&self) -> *const AllocatorBlock {
        self as *const AllocatorBlock
    }

    fn payload_mut_ptr(&mut self) -> *mut u8 {
        (unsafe { (self as *mut AllocatorBlock).byte_add(ALLOCATOR_BLOCK_SIZE) }) as *mut u8
    }

    fn raw_size(&self, heap_end: usize) -> usize {
        if let Some(next) = self.next {
            unsafe { next.byte_offset_from(self.ptr()).try_into().unwrap() }
        } else {
            heap_end - self.ptr() as usize
        }
    }

    fn payload_size(&self, heap_end: usize) -> usize {
        self.raw_size(heap_end) - ALLOCATOR_BLOCK_SIZE
    }

    unsafe fn split(&mut self, size: usize, heap_end: usize) -> Option<*mut AllocatorBlock> {
        if self.payload_size(heap_end) < size + ALLOCATOR_BLOCK_SIZE + MIN_BLOCK_USEFUL_SIZE {
            return None;
        }

        let new_next = self.payload_mut_ptr().byte_add(size) as *mut AllocatorBlock;
        (*new_next).next = self.next;
        (*new_next).prev = Some(self);
        (*new_next).used = false;

        if let Some(old_next) = self.next {
            (*old_next).prev = Some(new_next);
        }

        self.next = Some(new_next);

        Some(new_next)
    }

    unsafe fn merge_with_next(&mut self) {
        if let Some(next) = self.next
            && !(*next).used
        {
            if let Some(next) = (*next).next {
                (*next).prev = Some(self);
            }

            self.next = (*next).next;
        }
    }

    unsafe fn merge_with_prev(&mut self) {
        if !self.used
            && let Some(prev) = self.prev
            && !(*prev).used
        {
            if let Some(next) = self.next {
                (*next).prev = Some(prev);
            }

            (*prev).next = self.next;
        }
    }
}

pub struct DosAllocator {
    head: *mut AllocatorBlock,
    heap_end: usize,
}

impl DosAllocator {
    const fn new() -> Self {
        Self {
            head: null_mut(),
            heap_end: 0,
        }
    }

    pub fn init(&mut self) {
        let seg = unsafe { get_data_seg() };
        let off = &raw const _heap as u16 as usize;

        self.head = align_up(off, ALLOCATOR_ALIGN) as *mut AllocatorBlock;
        self.heap_end = LAST_MEMORY_BYTE_ADDR.saturating_sub((seg as usize) << 4);

        unsafe {
            *(self.head) = AllocatorBlock {
                next: None,
                prev: None,
                used: false,
            };
        }
    }
}

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut curr = self.head;
        let size = align_up(layout.size(), ALLOCATOR_ALIGN);

        while (*curr).used || (*curr).payload_size(self.heap_end) < size {
            match (*curr).next {
                Some(ptr) => curr = ptr,
                None => return null_mut(),
            };
        }

        (*curr).used = true;
        (*curr).split(size, self.heap_end);
        (*curr).payload_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let curr = ptr.byte_sub(ALLOCATOR_BLOCK_SIZE) as *mut AllocatorBlock;
        (*curr).used = false;
        (*curr).merge_with_next();
        (*curr).merge_with_prev();
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert_ne!(ptr, null_mut());

        let new_size = align_up(new_size, ALLOCATOR_ALIGN);
        let curr = ptr.byte_sub(ALLOCATOR_BLOCK_SIZE) as *mut AllocatorBlock;

        if new_size <= (*curr).payload_size(self.heap_end) {
            if let Some(new_next) = (*curr).split(new_size, self.heap_end) {
                (*new_next).merge_with_next();
            }

            return ptr;
        }

        let new_ptr = self.alloc(Layout::from_size_align(new_size, layout.align()).unwrap());

        if new_ptr.is_null() {
            return null_mut();
        }

        new_ptr.copy_from_nonoverlapping(ptr, layout.size().min(new_size));
        self.dealloc(ptr, layout);
        new_ptr
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[global_allocator]
pub(crate) static mut GLOBAL_ALLOCATOR: DosAllocator = DosAllocator::new();
