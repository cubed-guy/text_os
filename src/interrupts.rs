// Allow x86 interrupt calling convention
use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint  // An entry in idt
			.set_handler_fn(breakpoint_handler);
		idt.double_fault  // double fault entry in idt
			.set_handler_fn(double_fault_handler);
		// idt.double_fault.set_ist_index(DOUBLE_FAULT_IST_INDEX);

		idt
	};
}

pub fn init_idt() {
	IDT.load();
}


// Handling breakpoint exceptions

use x86_64::structures::idt::InterruptStackFrame;
use crate::println;

extern "x86-interrupt" fn breakpoint_handler(
	stack_frame: InterruptStackFrame
) {
	println!("WE JUSt HAD A BREAKPOINT EXCEPTION:\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
	x86_64::instructions::interrupts::int3();
}


extern "x86-interrupt" fn double_fault_handler(
	stack_frame: InterruptStackFrame, _error_code: u64
) -> ! { // diverging, x64 does not allow returning from double faults
	println!("The double fault exception handler was called.");

	// We need to switch stacks to prevent a stack overflow


	panic!("AAAAAAAAHHH!! DOUBLE FAULT!\n{:#?}", stack_frame);
}

