use x86_64::VirtualAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 4;  // arbitrary, between 0-7

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table  // the tss contains our interrupt stack table
			[DOUBLE_FAULT_IST_INDEX as usize] = {
				const STACK_SIZE: usize = 1<<16;
				static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

				let stack_start = VirtualAddr::from_ptr(unsafe { &STACK })
				let stack_end = stack_start + STACK_SIZE;
				stack_end  // the stack grows from here
			};
		tss
	}
}
