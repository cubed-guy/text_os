#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

struct ListNode {
	size: usize,
	next: Option<&'static mut Self>,
}

impl ListNode {
	const fn new(size: usize) -> Self {
		ListNode {size, next: None}
	}

	fn start_addr(&self) -> usize {
		self as *const Self as usize
	}

	fn end_addr(&self) -> usize {
		self.start_addr() + self.size
	}
}

struct LinkedListAllocator {
	head: ListNode,
}

use core::mem;
use super::align_up;

impl LinkedListAllocator {
	pub const fn new() -> Self {
		Self {
			head: ListNode::new(0),
		}
	}

	/// Initialising allocators is always unsafe.
	/// The caller must guarantee that the heap bounds are valid and unused.
	/// Must only be called once.
	pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
		self.add_free_region(heap_start, heap_size);
	}

	unsafe fn add_free_region(&mut self, addr: usize, size: usize) {

		// make sure the input is aligned
		assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);

		// make sure the freed region is big enough for node data
		assert!(size >= mem::size_of::<ListNode>());


		let mut node = ListNode::new(size);

		node.next = self.head.next.take();
		let node_ptr = addr as *mut ListNode;

		node_ptr.write(node);
		self.head.next = Some(&mut *node_ptr);
	}

	fn find_region(&mut self, size: usize, align: usize)
		-> Option<(&'static mut ListNode, usize)>  // node and the address
	{
		let mut current = &mut self.head;  // dummy

		while let Some(ref mut region) = current.next {
			if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
				let new_next = region.next.take();
				let ret = Some((current.next.take().unwrap(), alloc_start));
				current.next = new_next;  // retopologise the list
				return ret;
			} else {
				// we are in a good state
				// now I want to set current to the next pointer
				current = current.next.as_mut().unwrap();
			}
		}

		None
	}

	fn alloc_from_region(region: &ListNode, size: usize, align: usize)
		-> Result<usize, ()>
	{
		let alloc_start = align_up(region.start_addr(), align);
		let alloc_end = alloc_start.checked_add(size).ok_or(())?;

		if alloc_end > region.end_addr() {
			return Err(());
		}

		let remaining_size = region.end_addr() - alloc_end;
		if 0 < remaining_size && remaining_size < mem::size_of::<ListNode>() {
			return Err(());
		}

		Ok(alloc_start)
	}
}

use alloc::alloc::{GlobalAlloc, Layout};
use super::Locked;

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		use core::ptr;

		let (size, align) = LinkedListAllocator::size_align(layout);

		let mut allocator = self.lock();

		if let Some((region, alloc_start)) = allocator.find_region(size, align) {
			let alloc_end = alloc_start.checked_add(size).expect("overflow while allocating a region");

			let excess_size = region.end_addr() - alloc_end;

			if excess_size > 0 {
				allocator.add_free_region(alloc_end, excess_size);
			}

			alloc_start as *mut u8
		} else {
			ptr::null_mut()
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let (size, _) = LinkedListAllocator::size_align(layout);

		self.lock().add_free_region(ptr as usize, size)
	}
}

impl LinkedListAllocator {
	fn size_align(layout: Layout) -> (usize, usize) {
		let layout = layout
			.align_to(mem::align_of::<ListNode>())
			.expect("Could not align to ListNode")
			.pad_to_align();
		let size = layout.size().max(mem::size_of::<ListNode>());
		(size, layout.align())
	}
}
