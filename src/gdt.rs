use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 4;  // arbitrary, between 0-7

// we are adding an entry to the interrupt stack table to hold
// a different address for the stack of the double fault handler

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table  // the tss contains our interrupt stack table
			[DOUBLE_FAULT_IST_INDEX as usize] = {
				const STACK_SIZE: usize = 1<<16;
				static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

				let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
				let stack_end = stack_start + STACK_SIZE;
				stack_end  // the stack grows from here
			};
		tss
	};
}

// Creating the gdt

use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

lazy_static! {
	static ref GDT: (GlobalDescriptorTable, Selectors) = {  // Used for switching kernel-user mode and loading the TSS
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
		(gdt, Selectors { code_selector, tss_selector })
	};
}

struct Selectors {
	code_selector: SegmentSelector,
	tss_selector: SegmentSelector,
}

pub fn init() {
	GDT.0.load();  // loads it for hardware use
	// This is not enough, since the segment and TSS registers
	// are not updated with the new table values.
	// Point to the gdt, start using the new tss, update the idt

	use x86_64::instructions::tables::load_tss;
	use x86_64::instructions::segmentation::{CS, Segment};

	unsafe {
		CS::set_reg(GDT.1.code_selector);
		load_tss(GDT.1.tss_selector);
	}

}
