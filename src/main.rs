#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello,_World!";  // this is where our string lives

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    println!("Hello, {}", "World!");

    #[cfg(test)]
    test_main();

    // panic!("Oh noes!");
    loop {}
}

// specified because no std
#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAIL]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

// specified because no std
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    if tests.len() != 1 { serial_println!("Running {} tests!", tests.len()) }
    else { serial_println!("Running 1 test!") }

    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success); // all tests passed!
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


// defining exit codes, will use these to exit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {  // exit code from the qemu program = (e<<1)|1
    Success = 0x10,  // 0x21 (33)
    Failed = 0x11,   // 0x23 (35)
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // Port::new is unsafe? port.write() is unsafe? both
        // The 0xf4 port address is mapped to
        // the isa-debug-exit device.
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);  // u32 cuz iosize=0x04
    }
}

// The `Testable` Trait
trait Testable {
    fn run(&self);
}

impl<T> Testable for T where
    T: Fn(),
{
    fn run(&self) {
        // Type name of a function is the name of the function
        use core::any;
        serial_print!("{} is running:\t", any::type_name::<T>());

        self();
        serial_println!("[ok]");
    }
}
