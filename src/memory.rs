use x86_64::{
	structures::paging::PageTable,
	VirtAddr,
	PhysAddr,
};

/// offset must be valid
/// if called twice or more, will violate single mut ref
unsafe fn curr_l4_table(offset: VirtAddr)
	-> &'static mut PageTable
{
	use x86_64::registers::control::Cr3;
	let (level_4_table_frame, _) = Cr3::read();  // data of current page table

	let physical_address = level_4_table_frame.start_address();
	let virtual_address: VirtAddr = offset + physical_address.as_u64();
	let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

	&mut *page_table_ptr  // unsafe because we assume this is static
}

#[allow(dead_code)]
unsafe fn translate_address(address: VirtAddr, offset: VirtAddr)
	-> Option<PhysAddr>
{
	translate_address_inner(address, offset)
}

// Get called by unsafe function. Private and only translate_address() should use it
fn translate_address_inner(address: VirtAddr, offset: VirtAddr)
	-> Option<PhysAddr>
{
	use x86_64::structures::paging::page_table::FrameError;
	use x86_64::registers::control::Cr3;


	let table_indices = [
		address.p4_index(), address.p3_index(), address.p2_index(), address.p1_index()
	];

	let (level_4_table_frame, _) = Cr3::read();
	let mut frame = level_4_table_frame;

	for &index in &table_indices {
		let virt = offset + frame.start_address().as_u64();
		let table_ptr: *const PageTable = virt.as_ptr();
		let table = unsafe { &*table_ptr };


		let entry = &table[index];
		frame = match entry.frame() {
			Ok(frame) => frame,  // frame = entry.frame().unwrap()
			Err(FrameError::FrameNotPresent) => return None,
			Err(FrameError::HugeFrame) => panic!("Huge frames aren't supported yet"),
		}

	}

	Some(frame.start_address() + u64::from(address.page_offset()))

}


// Using an existing implementation
use x86_64::structures::paging::OffsetPageTable;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
	let l4_table = curr_l4_table(physical_memory_offset);
	OffsetPageTable::new(l4_table, physical_memory_offset)
}
