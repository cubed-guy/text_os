#![no_std]
#![no_main]


use core::panic::PanicInfo;
use text_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("\x1b[92m[ok]\x1b[0m");
    exit_qemu(QemuExitCode::Success);
    loop {}
}


// add a _start function and other missing parts


#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("\x1b[91m[test did not panic]\x1b[0m");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


// the first and only test case in this environment
fn should_fail() {
    serial_print!("should_panic::should_fail:\t");
    assert_eq!(0, 1);
}
