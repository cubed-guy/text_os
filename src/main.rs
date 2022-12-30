#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello,_World!";  // this is where our string lives

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    vga_buffer::yet_another_printer();

    loop {}
}

// specified because no std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
