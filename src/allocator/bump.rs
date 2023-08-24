pub struct BumpAllocator {
	heap_start: usize,
	heap_end: usize,
	next: usize,
	alloc_count: usize,
}

/// The interface has been made to match that of linked_list_allocator
/// This way, we can 
impl BumpAllocator {
	pub const fn new() -> Self {
		BumpAllocator {
			heap_start: 0,
			heap_end: 0,
			next: 0,
			alloc_count: 0,
		}
	}

	/// Initialises the BumpAllocator with given memory region for the heap
	///
	/// It's unsafe because it assumes the region to be unused.
	/// Also, this method must be called only once for each instance.
	pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
		self.heap_start = heap_start;
		self.heap_end = heap_end;
		self.next = heap_start;
	}
}

use alloc::alloc::{GlobalAlloc, Layout};
use super::Locked;
use core::ptr;

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut locked_self = self.inner.lock();

		// TODO: alignment
		let alloc_start = locked_self.next;
		locked_self.alloc_count += 1;
		if locked_self.next + layout.size() > locked_self.heap_end {
			ptr::null_mut()
		} else {
			locked_self.next = alloc_start + layout.size();
			alloc_start as *mut u8
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		let mut locked_self = self.lock();

		locked_self.alloc_count -= 1;
		if locked_self.alloc_count == 0 {
			locked_self.next = locked_self.heap_start;
		}
	}
}
