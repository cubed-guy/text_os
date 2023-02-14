// all the crate attributes again
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(text_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	test_main();

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	text_os::test_panic_handler(info)
}

use text_os::println;

// We test this here because it's a basic environment.
// If println breaks, we'll know that.
#[test_case]
fn print_simple() {
	println!("Hello, {}", "QEMU");
}
