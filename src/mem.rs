//! Memory heap allocation for DOS programs.
//! Uses conventional memory for DOS programs, from _heap segment start to extended BIOS data area (EBDA)
//! Uses linear algorithm for allocating memory, which is not optimal, but it's simple and works.

use core::{
    alloc::{GlobalAlloc, Layout},
    cmp::min,
    mem::size_of,
    ptr::null_mut,
};

use crate::dos::get_data_seg;

const LAST_MEMORY_BYTE_ADDR: usize = 0x9FBFF; // (0X9000 << 4) + 0xFBFF, last byte of memory before extended BIOS data area
const ALLOCATOR_BLOCK_SIZE: usize = size_of::<AllocatorBlock>();
const MIN_BLOCK_USEFUL_SIZE: usize = 16;

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

    fn raw_size(&self) -> usize {
        if let Some(next) = self.next {
            unsafe { next.byte_offset_from(self.ptr()).try_into().unwrap() }
        } else {
            LAST_MEMORY_BYTE_ADDR - self.ptr() as usize
        }
    }

    fn payload_size(&self) -> usize {
        self.raw_size() - ALLOCATOR_BLOCK_SIZE
    }
}

pub struct DosAllocator {
    head: *mut AllocatorBlock,
}

impl DosAllocator {
    const fn new() -> Self {
        Self { head: null_mut() }
    }

    pub fn init(&mut self) {
        let seg = unsafe { get_data_seg() };
        let off = core::ptr::addr_of!(_heap) as u16 as usize;
        self.head = (((seg as usize) << 4) + off) as *mut AllocatorBlock;

        unsafe {
            *(self.head) = AllocatorBlock {
                next: None,
                prev: None,
                used: false,
            };
        }
    }
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut curr = self.head;

        while (*curr).used || (*curr).payload_size() < layout.size() {
            match (*curr).next {
                Some(ptr) => curr = ptr,
                None => return null_mut(),
            };
        }

        if (*curr).payload_size() == layout.size() {
            (*curr).used = true;
            return (*curr).payload_mut_ptr();
        }

        let new_next = curr.byte_add(layout.size()).byte_add(ALLOCATOR_BLOCK_SIZE);
        (*new_next).next = (*curr).next;
        (*new_next).prev = Some(curr);
        (*new_next).used = false;

        if let Some(old_next) = (*curr).next {
            (*old_next).prev = Some(new_next);
        }

        (*curr).next = Some(new_next);
        (*curr).used = true;

        (*curr).payload_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // Freeing null pointer is a no-op
        if ptr == null_mut() {
            return;
        }

        let current_block_ptr = (ptr as u32 - ALLOCATOR_BLOCK_SIZE as u32) as *mut AllocatorBlock;
        // Mark block as free
        (*current_block_ptr).used = false;

        // Merge with next block if it's free
        let next_block_ptr = (*current_block_ptr).next;
        if next_block_ptr.is_some() {
            let next_block_ptr = next_block_ptr.unwrap();
            if !(*next_block_ptr).used {
                if (*next_block_ptr).next.is_some() {
                    (*(*next_block_ptr).next.unwrap()).prev = Some(current_block_ptr);
                }
                (*current_block_ptr).next = (*next_block_ptr).next;
            }
        }

        // Merge with previous block if it's free
        let prev_block_ptr = (*current_block_ptr).prev;
        if prev_block_ptr.is_some() {
            let prev_block_ptr = prev_block_ptr.unwrap();
            if !(*prev_block_ptr).used {
                if (*current_block_ptr).next.is_some() {
                    (*(*current_block_ptr).next.unwrap()).prev = Some(prev_block_ptr);
                }
                (*prev_block_ptr).next = (*current_block_ptr).next;
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert_ne!(ptr, null_mut());

        let curr = (ptr as usize - ALLOCATOR_BLOCK_SIZE) as *mut AllocatorBlock;

        if new_size <= (*curr).payload_size() {
            // TODO: make a vacant block from the space left (if this feasible i.e. >= ALLOCATOR_BLOCK_SIZE + MIN_BLOCK_USEFUL_SIZE)
            return ptr;
        }

        let new_ptr = self.alloc(Layout::from_size_align(new_size, layout.align()).unwrap());
        new_ptr.copy_from_nonoverlapping(ptr, min(layout.size(), new_size));
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
