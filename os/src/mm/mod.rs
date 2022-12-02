mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;
mod memory_set;

use page_table::{PageTable, PTEFlags};
pub use frame_allocator::{FrameTracker, frame_alloc, frame_dealloc};
pub use page_table::{PageTableEntry, translated_byte_buffer};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission};
pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum, StepByOne};
pub use frame_allocator::{FrameTracker, frame_alloc};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
