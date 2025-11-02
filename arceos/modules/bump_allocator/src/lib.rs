#![no_std]

use core::num;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    count: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            count: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = self.start;
        self.p_pos = (self.end / SIZE) * SIZE; // align downward
        self.count = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        unimplemented!("EarlyAllocator does not support add_memory")
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        // Align b_pos upward, align has been guaranteed to be power of 2 by Layout
        let align = layout.align();
        let original_b_pos = self.b_pos;
        self.b_pos = (self.b_pos + align - 1) & !(align - 1);
        // Check available bytes
        if self.available_bytes() < layout.size() {
            self.b_pos = original_b_pos;
            return Err(allocator::AllocError::NoMemory);
        }
        // Increment allocation count
        self.count += 1;
        let addr = self.b_pos;
        self.b_pos += layout.size();
        Ok(core::ptr::NonNull::new(addr as *mut u8).unwrap())
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        // Check valid address
        let addr = pos.as_ptr() as usize;
        // Ensure the address is within the allocated byte range
        if addr < self.start || addr + layout.size() > self.b_pos {
            panic!("EarlyAllocator dealloc invalid address");
        }
        // Note: EarlyAllocator can't check if one allocation is freed multiple times.

        // Decrement allocation count
        self.count -= 1;
        if self.count == 0 {
            // Free all byte allocations
            self.b_pos = self.start;
        } else {
            // Only free the last allocation, increase the limited utilization slightly
            if addr + layout.size() == self.b_pos {
                self.b_pos -= layout.size();
            }
        }
    }

    fn total_bytes(&self) -> usize {
        self.p_pos - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        let original_p_pos = self.p_pos;
        // Align p_pos downward
        self.p_pos -= num_pages * Self::PAGE_SIZE;
        self.p_pos &= !(align_pow2 - 1);

        // Check available pages
        if self.available_pages() < num_pages {
            self.b_pos = original_p_pos;
            return Err(allocator::AllocError::NoMemory);
        }

        // Alloc pages
        Ok(self.p_pos)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        unimplemented!("EarlyAllocator does not support dealloc_pages")
    }

    fn total_pages(&self) -> usize {
        (self.end - self.b_pos) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        self.available_bytes() / Self::PAGE_SIZE
    }
}
