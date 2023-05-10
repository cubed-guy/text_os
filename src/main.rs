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

use bootloader::{BootInfo, entry_point};

// creates a declaration for an entry point function.
// defines _start here itself
entry_point!(kernel_main);

// entry point before init of runtime
fn kernel_main(_boot_info: &'static BootInfo) -> ! {  // '!' never returns
    println!("Hello, {}", "World!");

    text_os::init();  // calls all the init methods

    // // invoking an interrupt breakpoint exception explicitly
    // x86_64::instructions::interrupts::int3();  // Is this what an intrinsic is?

    // causing a page fault when there is no page fault handler
    // but now we do have a handler
    // unsafe {
    //     println!("Making an unsafe dereference");
    //     *(0xdeadbeef as *mut u32) = 42;
    // }
    // let ptr = 0xdeadbeef as *mut u32;
    // unsafe { core::ptr::write_unaligned(ptr, 42); }
    // // unsafe { *ptr = 42; }  // works in release build
    // A stack overflow causes a triple fault if there's no stack switching.

    let ptr = 0x22259b as *mut u32;
    unsafe { let _x = core::ptr::read_unaligned(ptr); }
    println!("Read from the instruction pointer worked!");

    // unsafe { core::ptr::write_unaligned(ptr, 42); }
    // println!("Write to the instruction pointer worked!");

    use x86_64::registers::control::Cr3;  // points to the current page table

    // (physical frame, flags)
    let (level_4_page_table, _) = Cr3::read();
    println!("Physical Address of the current page table: {:?}",
        level_4_page_table.start_address());


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
