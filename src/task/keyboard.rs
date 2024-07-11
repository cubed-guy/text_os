use crossbeam_queue::ArrayQueue;
use conquer_once::spin::OnceCell;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();


/// Call if you want to add a key event to the global key queue
/// Must not block or allocate
/// pub(crate) visibility means it is public only within the crate
pub(crate) fn update_scancode_queue(scancode: u8) {
	use crate::println;

	let scancode_queue_res = SCANCODE_QUEUE.try_get();
	if let Ok(queue) = scancode_queue_res {
		let push_res = queue.push(scancode);
		if let Err(_) = push_res {
			println!("WARNING: SCANCODE QUEUE IS FULL")
		}
	}
	else {
		println!("WARNING: SCANCODE QUEUE IS NOT INITIALISED")
	}
}
