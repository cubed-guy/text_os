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

	fn list_index(&self, layout: &Layout) -> Option<usize> {
		let required_size = layout.size().max(layout.align());
		BLOCK_SIZES.iter().position(|&s| s >= required_size)
	}
}

use super::Locked;
use alloc::alloc::GlobalAlloc;

unsafe impl GlobalAlloc for Locked<FixedSizeAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut allocator = self.lock();
		match allocator.list_index(&layout) {
			None => allocator.fallback_alloc(layout),
			Some(index) => {
				match allocator.list_heads[index].take() {
					Some(node) => {
						// pop the list stack
						allocator.list_heads[index] = node.next.take();
						node as *mut ListNode as *mut u8
					}
					None => {
						let block_size = BLOCK_SIZES[index];
						let block_align = block_size;
						let layout =
							Layout::from_size_align(block_size, block_align)
							.unwrap();
						allocator.fallback_alloc(layout)
					}
				}
			}
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let mut allocator = self.lock();

		match allocator.list_index(&layout) {
			// size not in default blocks, dealloc using fallback
			None => {
				use core::ptr::NonNull;
				let ptr = NonNull::new(ptr).unwrap();

				allocator.fallback_allocator.deallocate(ptr, layout);
			}

			// dealloc from blocks, add to list
			Some(index) => {
				// this is what we want to put in the block
				let list_node = ListNode{
					// it's a <&'static mut ListNode>
					next: allocator.list_heads[index].take()
				};

				assert!(layout.size() <= BLOCK_SIZES[index]);  // size check
				assert!(layout.align() <= BLOCK_SIZES[index]);  // align check

				// list_node is on the heap
				// we need to copy it onto the heap block
				let ptr = ptr as *mut ListNode;  // cast from *mut u8
				ptr.write_volatile(list_node);   // point block to old head

				// set new head
				allocator.list_heads[index] = Some(&mut *ptr);  // unsafe deref
			}
		}
	}
}
