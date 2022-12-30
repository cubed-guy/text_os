#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

use core::convert::TryFrom;
use core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello,_World!";  // this is where our string lives

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    let vga_buffer = 0xb8020 as *mut u8;  // raw pointer to vga_buffer
    // it is known to be at address 0x8000

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {  // to dereference the raw pointer
            *vga_buffer.offset(i as isize * 2) = byte;  // character at 2i  (0, 2, 4...)
            *vga_buffer.offset(i as isize * 2 + 1) = ((15 - i%16 << 4)+i%16) as u8;
        }
    }
    // 0x8000: 'H', 0xb, 'e', 0xb, 'l', 0xb...

    loop {}
}

// specified because no std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
