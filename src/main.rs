#![no_std] // std requires OS specific things
#![no_main] // otherwise assumes a runtime, which requires an OS

#![feature(custom_test_frameworks)]
#![test_runner(text_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
// use text_os::println;

// static HELLO: &[u8] = b"Hello,_World!";  // this is where our string lives

// entry point before init of runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {  // '!' never returns
    println!("Hello, {}", "World!");

    text_os::init();  // calls all the init methods

    // // invoking an interrupt breakpoint exception explicitly
    // x86_64::instructions::interrupts::int3();  // Is this what an intrinsic is?

    // causing a page fault when there is no page fault handler
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    // fn stack_overflow(n: i32) {
    //     if n != 0 {
    //         stack_overflow(n);
    //     }
    // }

    // stack_overflow(1);

    #[cfg(test)]
    test_main();


    println!("There was an exception maybe? But it didn't crash.");
    // panic!("Oh noes!");
    loop {}
}

// specified because no std
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// specified because no std
#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    text_os::test_panic_handler(info);
}
