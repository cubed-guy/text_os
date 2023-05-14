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
fn kernel_main(boot_info: &'static BootInfo) -> ! {  // '!' never returns
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

    println!(
        "We're mapping physical memory to virtual memory using this offset: {:x}",
        boot_info.physical_memory_offset
    );
    // unsafe { core::ptr::write_unaligned(ptr, 42); }
    // println!("Write to the instruction pointer worked!");

    use text_os::memory::init;
    use x86_64::VirtAddr;

    let physical_memory_offset =
        VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { init(physical_memory_offset) };

    // for (i, entry) in mapper.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 [{:2}] {:?}", i, entry);

    //         use x86_64::structures::paging::PageTable;
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr: *mut PageTable = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 [{:2}] {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    use x86_64::registers::control::Cr3;  // points to the current page table

    // (physical frame, flags)
    let (level_4_page_table, _) = Cr3::read();
    println!("Physical Address of the current page table: {:?}",
        level_4_page_table.start_address());

    use x86_64::structures::paging::Translate;

    let addresses = [
        // vga buffer
        0xb8000,

        // code page... how do we know this?
        0x201008,

        // stack page... again, how do we know this?
        0x0100_0020_1a10,

        // physical address 0
        boot_info.physical_memory_offset,

    ];

    for &address in &addresses {  // also what's this for loop syntax
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }


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
