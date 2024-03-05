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
		use super::align_up;
		use core::mem;

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

}
