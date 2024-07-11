use crossbeam_queue::ArrayQueue;
use conquer_once::spin::OnceCell;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
const SCANCODE_QUEUE_CAP: usize = 0x80;

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
		} else {
			WAKER.wake();
		}
	}
	else {
		println!("WARNING: SCANCODE QUEUE IS NOT INITIALISED")
	}
}

pub struct ScancodeStream {
	_private: ()
}

impl ScancodeStream {
	pub fn new() -> Self {
		SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(SCANCODE_QUEUE_CAP))
			.expect("Scancode Queue must be iniitialised only once");
		ScancodeStream{_private: ()}
	}
}

use core::pin::Pin;
use core::task::Poll;
use core::task::Context;
use futures_util::Stream;
impl Stream for ScancodeStream {
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE.try_get().expect("Queue not initialised");

		if let Some(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode))
		}

		WAKER.register(&ctx.waker());

		match queue.pop() {
			Some(scancode) => {
				WAKER.take();
				Poll::Ready(Some(scancode))
			}
			None => Poll::Pending,
		}
	}
}

use futures_util::task::AtomicWaker;
static WAKER: AtomicWaker = AtomicWaker::new();
