#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use text_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}


// add a _start function and other missing parts


#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    if tests.len() != 1 { serial_println!("Running {} tests!\t", tests.len()) }
    else { serial_println!("Running 1 test!") }
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}


// the first and only test case in this environment
#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail:\t");
    assert_eq!(0, 1);
}
