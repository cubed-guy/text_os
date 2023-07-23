// lib.rs is a specially managed file by cargo

#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where
    T: Fn(),
{
    fn run(&self) {
        use core::any;
        serial_print!("Running test {}:\t", any::type_name::<T>());
        self();
        serial_println!("\x1b[92m[ok]\x1b[0m");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    if tests.len() != 1 { serial_println!("Running {} tests!\t", tests.len()) }
    else { serial_println!("Running 1 test!") }
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("\x1b[91m[FAIL]");
    serial_println!("Error:\x1b[0m {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}



#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);

// Entry point
#[cfg(test)]  // necessary because not always a test
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}



// Qemu Exit Functions

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

pub mod serial;
pub mod vga_buffer;


// Exceptions and Interrupts

pub mod interrupts;

// idt and all other things will be initialised here.
pub fn init() {
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();  // `sti` intrinsic, CPU will now listen for interrupts
}


// Stack switching

pub mod gdt;


// a better way to loop endlessly
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();  // thread sleeps until interrupt occurs
    }
}


// Paging!
pub mod memory;

extern crate alloc;
pub mod allocator;
