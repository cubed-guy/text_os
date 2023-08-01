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

unsafe impl GlobalAlloc for BumpAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let alloc_start = self.next;
		self.alloc_count += 1;
		self.next = alloc_start + layout.size();
		if self.next > self.heap_end {
			panic!("Out of memory")
		}
		alloc_start as *mut u8
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		if self.alloc_count < 0 {
			return
		}

		self.alloc_count -= 1;
		if self.alloc_count == 0 {
			self.next = self.heap_start;
		}
	}
}
