struct ListNode {
	next: Option<&'static mut ListNode>
}

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeAllocator {
	list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
	fallback_allocator: linked_list_allocator::Heap,
}

use alloc::alloc::Layout;

impl FixedSizeAllocator {
	pub const fn new() -> Self {
		const EMPTY: Option<&'static mut ListNode> = None;

		FixedSizeAllocator{
			list_heads: [EMPTY; BLOCK_SIZES.len()],
			fallback_allocator: linked_list_allocator::Heap::empty(),
		}
	}

	pub unsafe fn init(&mut self, heap_start: *mut u8, heap_size: usize) {
		self.fallback_allocator.init(heap_start, heap_size);
	}

	fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
		use core::ptr;

		match self.fallback_allocator.allocate_first_fit(layout) {
			Ok(ptr) => ptr.as_ptr(),
			Err(_) => ptr::null_mut(),
		}
	}

	fn list_index(&self, layout: Layout) -> Option<usize> {
		let required_size = layout.size().max(layout.align());
		BLOCK_SIZES.iter().position(|&s| s >= required_size)
	}
}
