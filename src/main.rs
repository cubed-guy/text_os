#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello,_World!";  // this is where our string lives

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    // vga_buffer::yet_another_printer();

    use core::fmt::Write;  // to be able to use write!()

    vga_buffer::WRITER.lock().write_str("Hello, World!").unwrap();
    write!(vga_buffer::WRITER.lock(), "\nNumbers: {} {} {}", 3, 1.4, 55.029).unwrap();

    loop {}
}

// specified because no std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
