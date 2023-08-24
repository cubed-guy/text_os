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


// Allocator designs

pub mod bump;

/// A wrapper around Mutex to be able implement traits for it.
/// Remember, its traits can be implemented anywhere in the same _crate_.
pub struct Locked<A> {
	pub inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
	pub const fn new(inner: A) -> Locked<A> {
		Locked {
			inner: spin::Mutex::new(inner),
		}
	}

	pub fn lock(&self) -> spin::MutexGuard<A> {
		self.inner.lock()
	}
}

/// we assume alignment is a power of two.
/// Maybe we should enforce it using the type system?
fn align_up(n: usize, alignment: usize) -> usize {
	// n == am => (am-1)|(a-1) + 1 = (am-1) + 1 = am
	// n == am + k => (am+k-1)|(a-1) + 1 = (am+a-1) + 1 = am+a
	((n - 1) | (alignment - 1)) + 1

	// n == am => (am+a-1) & !(a-1) = am
	// n == am + k => (am+k+a-1) & !(a-1) = (am+a + k-1) & !(a-1) = am+a
	// (n+a-1) & !(a-1)
}
