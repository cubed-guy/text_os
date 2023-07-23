/// Contains code for allocations.

pub struct Dummy();

use alloc::alloc::{GlobalAlloc, Layout};

unsafe impl GlobalAlloc for Dummy {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		use core::ptr::null_mut;
		null_mut()
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		panic!("Dealloc not allowed!")
	}

}


// // and here we tell Rust to use this as the allocator
// #[global_allocator]
// static ALLOCATOR: Dummy = Dummy();

pub const HEAP_START: *mut u8 = 0x4eab_a2ea_0000 as *mut u8;
pub const HEAP_SIZE:  usize = 0x2_0000;

use x86_64::structures::paging::PageTableFlags;
use x86_64::structures::paging::{Size4KiB, Mapper, FrameAllocator, mapper::MapToError, Page};
use x86_64::VirtAddr;

pub fn init_heap(
	mapper: &mut impl Mapper<Size4KiB>,
	frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {

	let heap_start = VirtAddr::new(HEAP_START as u64);
	let heap_end = heap_start + HEAP_SIZE - 1u64;
	let heap_start_page = Page::containing_address(heap_start);
	let heap_end_page = Page::containing_address(heap_end);
	let page_range = Page::range_inclusive(heap_start_page, heap_end_page);

	for page in page_range {
		let frame = frame_allocator
			.allocate_frame()
			.ok_or(MapToError::FrameAllocationFailed)?;
		let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
		unsafe {
			mapper.map_to(page, frame, flags, frame_allocator)?.flush();
		};
	}

	unsafe {
		ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
	}

	Ok(())  // source.rust meta.function.rust meta.block.rust support.type.rust
}

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
