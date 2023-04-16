#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn _start() -> ! {
	unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	text_os::test_panic_handler(info)
}
