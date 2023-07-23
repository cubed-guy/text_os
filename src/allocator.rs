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


// and here we tell Rust to use this as the allocator
#[global_allocator]
static ALLOCATOR: Dummy = Dummy();
