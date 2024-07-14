use core::fmt;
use core::{future::Future, pin::Pin};
use alloc::boxed::Box;

pub struct Task {
	id: TaskId,
	future: Pin<Box<dyn Future<Output = ()>>>
}

use core::task::{Context, Poll};

impl Task {
	pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
		Task {
			id: TaskId::new(),
			future: Box::pin(future)
		}
	}

	// private because it must be called only by the *Executor*
	fn poll(&mut self, context: &mut Context) -> Poll<()> {
		self.future.as_mut().poll(context)
	}
}

impl fmt::Debug for Task {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f
			.debug_struct("Task")
			.field("future", &"unknown")
			.finish()
	}
}

pub mod basic_executor;
pub mod keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
	fn new() -> Self {
		use core::sync::atomic::{AtomicU64, Ordering};
		static NEXT_ID: AtomicU64 = AtomicU64::new(0);
		Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
	}
}

pub mod better_executor;
