use super::Task;
use alloc::collections::VecDeque;

pub struct BasicExecutor {
	task_queue: VecDeque<Task>
}

impl BasicExecutor {
	pub fn new() -> BasicExecutor {
		BasicExecutor { task_queue: VecDeque::new() }
	}

	pub fn spawn(&mut self, task: Task) {
		self.task_queue.push_back(task);
	}

	pub fn run(&mut self) {
		while let Some(mut task) = self.task_queue.pop_front() {
			use core::task::{Poll, Context};
			let waker = create_dummy_waker();
			let mut ctx = Context::from_waker(&waker);
			match task.poll(&mut ctx) {
				Poll::Ready(()) => (),
				Poll::Pending => self.task_queue.push_back(task),
			};
		}

		use crate::println;
		println!("Executor completed")
	}
}

// Waker Stuff
use core::task::{Waker, RawWaker};

fn dummy_raw_waker() -> RawWaker {
	use core::task::RawWakerVTable;

	fn no_op(_: *const ()) {}
	fn clone(_: *const ()) -> RawWaker {
		dummy_raw_waker()
	}

	let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);

	// Null ptr to be passed as waker data
	let null_ptr = 0 as *const ();

	RawWaker::new(null_ptr, vtable)
}

fn create_dummy_waker() -> Waker {
	unsafe { Waker::from_raw(dummy_raw_waker()) }
}
