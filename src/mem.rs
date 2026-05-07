//! Memory heap allocation for DOS programs.
//! Uses conventional memory for DOS programs, from the EXE stack end to extended BIOS data area (EBDA)
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
const ALLOCATOR_ALIGN: usize = 16;
const EXE_STACK_SIZE: usize = 0x1000;

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

        self.head = align_up(off + EXE_STACK_SIZE, ALLOCATOR_ALIGN) as *mut AllocatorBlock;
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

        if (*curr).payload_size(self.heap_end) < size + ALLOCATOR_BLOCK_SIZE + MIN_BLOCK_USEFUL_SIZE
        {
            (*curr).used = true;
            return (*curr).payload_mut_ptr();
        }

        let new_next = curr.byte_add(size).byte_add(ALLOCATOR_BLOCK_SIZE);
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
        if ptr.is_null() {
            return;
        }

        let curr = (ptr as u32 - ALLOCATOR_BLOCK_SIZE as u32) as *mut AllocatorBlock;
        (*curr).used = false;

        // Merge with next block if it's free
        if let Some(next) = (*curr).next
            && !(*next).used
        {
            if (*next).next.is_some() {
                (*(*next).next.unwrap()).prev = Some(curr);
            }
            (*curr).next = (*next).next;
        }

        // Merge with previous block if it's free
        if let Some(prev) = (*curr).prev
            && !(*prev).used
        {
            if (*curr).next.is_some() {
                (*(*curr).next.unwrap()).prev = Some(prev);
            }
            (*prev).next = (*curr).next;
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert_ne!(ptr, null_mut());

        let curr = (ptr as usize - ALLOCATOR_BLOCK_SIZE) as *mut AllocatorBlock;

        if new_size <= (*curr).payload_size(self.heap_end) {
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
