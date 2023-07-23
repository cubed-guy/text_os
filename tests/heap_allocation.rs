#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(text_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
	text_os::test_panic_handler(info)
}

fn main(boot_info: &'static BootInfo) -> ! {
	use text_os::allocator;
	use text_os::memory::{self, BootInfoFrameAllocator};
	use x86_64::VirtAddr;

	text_os::init();

	let page_table_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe {
		memory::init(page_table_offset)
	};
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Unable to allocate heap");

	test_main();
	loop {}
}

#[test_case]
fn simple_allocation() {
	use alloc::boxed::Box;
	let val1 = Box::new(0xff9088);
	let val2 = Box::new(56);
	assert_eq!(*val1, 0xff9088);
	assert_eq!(*val2, 56);
}

#[test_case]
fn large_vec() {
	use alloc::vec::Vec;
	let n = 1000;
	let mut vec = Vec::new();
	for i in 0..n {
		vec.push(i);
	}
	assert_eq!(vec.iter().sum::<u64>(), n * (n-1) / 2);
}

#[test_case]
fn many_boxes() {
	use text_os::allocator::HEAP_SIZE;
	use alloc::boxed::Box;
	for i in 1..HEAP_SIZE {
		let x = Box::new(i);
		assert_eq!(*x, i);
	}
}
