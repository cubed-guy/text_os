#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

use core::panic::PanicInfo;

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    loop {}
}

// specified because no std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
