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
    // but now we do have a handler
    // unsafe {
    //     println!("Making an unsafe dereference");
    //     *(0xdeadbeaf as *mut u32) = 42;
    // }
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { core::ptr::write_unaligned(ptr, 42); }
    // A stack overflow causes a triple fault if there's no stack switching.

    #[cfg(test)]
    test_main();


    println!("There was an exception maybe? But it didn't crash.");
    // panic!("Oh noes!");
    // loop {
    //     // when an interrupt occurs,
    //     // the handler will wait for the writer to be unlocked.
    //     // this thread waits for the interrupt to end.
    //     // deadlock!
    //     for _ in 1..1000_000 { }
    //     print!("-");
    //     // The SOLUTION? Prevent interrupts when the mutex is locked.
    // }
    text_os::hlt_loop();
}

// specified because no std
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    text_os::hlt_loop();
}

// specified because no std
#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    text_os::test_panic_handler(info);
}
