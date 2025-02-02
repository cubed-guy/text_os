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
use text_os::allocator;

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

    use text_os::memory;
    use x86_64::VirtAddr;

    let physical_memory_offset =
        VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    use x86_64::structures::paging::Page;
    let page = Page::containing_address(VirtAddr::new(0xdead_beef));
    memory::create_example_mapping(
        page, &mut mapper, &mut frame_allocator
    );

    // Writing to the page table
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // The hex num represents a white "New!"
    unsafe { page_ptr.offset(266).write_volatile(0x_f021_f077_f065_f04e) };
    unsafe { page_ptr.offset(248).write_volatile(0x_f021_f077_f065_f04e) };

    use x86_64::registers::control::Cr3;  // points to the current page table

    // (physical frame, flags)
    let (level_4_page_table, _) = Cr3::read();
    println!("Physical Address of the current page table: {:?}",
        level_4_page_table.start_address());

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

    use x86_64::structures::paging::Translate;
    for &address in &addresses {  // also what's this for loop syntax
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }


    // Heap stuff

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialisation failed.");
    extern crate alloc;
    use alloc::{boxed::Box, vec::Vec, rc::Rc};
    use alloc::vec;

    // Box
    let heap_value = Box::new(42);
    println!("Value at the heap: {:p}.", heap_value);

    // Vec
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec is at {:p}", vec.as_slice());

    // Rc
    let ref_counted = Rc::new(vec![1, 2, 3]);
    let cloned_ref  = ref_counted.clone();
    println!("Before dropping, reference count = {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(ref_counted);
    println!("After dropping,  reference count = {}", Rc::strong_count(&cloned_ref));
    println!("rc is at {:p}", cloned_ref);

    // Async Stuff

    #[cfg(test)]
    test_main();

    use text_os::task::{Task, better_executor::Executor};
    let mut executor = Executor::new();

    println!("Will sleep when there's nothing to do.");

    executor.spawn(Task::new(another_example()));

    use text_os::task::keyboard::print_keypresses;
    executor.spawn(Task::new(print_keypresses()));

    executor.run();
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

// Some async stuff
async fn async_number() -> i32 {
    42
}

async fn another_example() {
    let n = async_number().await;
    println!("Async number: {}", n);
}
