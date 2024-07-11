use core::{future::Future, pin::Pin};
use alloc::boxed::Box;

pub struct Task {
	future: Pin<Box<dyn Future<Output = ()>>>
}

use core::task::{Context, Poll};

impl Task {
	pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
		Task {
			future: Box::pin(future)
		}
	}

	// private because it must be called only by the *Executor*
	fn poll(&mut self, context: &mut Context) -> Poll<()> {
		self.future.as_mut().poll(context)
	}
}

pub mod basic_executor;
pub mod keyboard;
