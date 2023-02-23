// Allow x86 interrupt calling convention
use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint  // An entry in idt
			.set_handler_fn(breakpoint_handler);

		idt
	};
}

pub fn init_idt() {
	IDT.load();
}


// Handling breakpoint exceptions

use x86_64::structures::idt::InterruptStackFrame;
use crate::println;
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("WE JUSt HAD A BREAKPOINT EXCEPTION: {:#?}", stack_frame);
}
