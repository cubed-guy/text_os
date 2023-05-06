// Allow x86 interrupt calling convention
use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

use crate::gdt;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint  // An entry in idt
			.set_handler_fn(breakpoint_handler);

		unsafe {
			idt.double_fault  // double fault entry in idt
				.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}

		// Hardware interrupt entries
		idt[InterruptIndex::Timer.as_usize()]
			.set_handler_fn(timer_interrupt_handler);
		idt[InterruptIndex::Keyboard.as_usize()]
			.set_handler_fn(keyboard_interrupt_handler);

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



// external interrupts

use pic8259::ChainedPics;
use spin;

// we'll be remapping the interrupt numbers from the PIC
// (PIC - programmable interrupt controller)

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =  // Mutex ie bottleneck lol
	spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });  // 32+0 to 32+15  (32-47)


// We'll make an enum to name the interrupt indices
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET,
	Keyboard,
}

impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}

extern "x86-interrupt" fn timer_interrupt_handler(
	_stack_frame: InterruptStackFrame,
) {
	use crate::print;
	print!(".");

	// Notify the PIC (not CPU) to end the interrupt and become available again
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
	_stack_frame: InterruptStackFrame,
) {
	use crate::print;
	print!("k");

	// Notify the PIC (not CPU) to end the interrupt and become available again
	// ----***  Don't forget to use the correct interrupt index!  ***----
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}
