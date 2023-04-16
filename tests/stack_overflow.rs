// This test fails abnormally on triple fault.

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use text_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_print!("stack_overflow::stack_overflow:\t");

	text_os::gdt::init();
	init_test_idt();  // not the default because we should return success

	stack_overflow();

	panic!("Execution continued past stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	text_os::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
	stack_overflow();
	volatile::Volatile::new(0).read();  // prevents tail recursion optimisation
}


// Creating the Test IDT

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();

		unsafe {  // setting the double fault stack index is unsafe
			idt.double_fault
			.set_handler_fn(test_double_fault_handler)
			.set_stack_index(text_os::gdt::DOUBLE_FAULT_IST_INDEX)
		};

		idt
	};
}

fn init_test_idt() {
	TEST_IDT.load();
}

use text_os::{QemuExitCode, exit_qemu, serial_println};

extern "x86-interrupt" fn test_double_fault_handler(
	_stack_frame: InterruptStackFrame,
	_error_code: u64,
) -> ! {
	serial_println!("\x1b[92m[ok]\x1b[0m");
	exit_qemu(QemuExitCode::Success);
	loop {}

}
