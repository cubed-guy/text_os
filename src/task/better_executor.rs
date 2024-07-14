use alloc::task::Wake;
use alloc::collections::BTreeMap;
use crate::task::{TaskId, Task};
use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;
use futures_util::task::Waker;

pub struct Executor {
	tasks: BTreeMap<TaskId, Task>,
	task_queue: Arc<ArrayQueue<TaskId>>,
	waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
	pub fn new() -> Self {
		Self {
			tasks: BTreeMap::new(),
			task_queue: Arc::new(ArrayQueue::new(100)),
			waker_cache: BTreeMap::new(),
		}
	}

	pub fn spawn(&mut self, task: Task) {
		// add task to queue, task already contains a taskId
		let task_id = task.id;
		if self.tasks.insert(task_id, task).is_some() {
			panic!("Inserting a task with existing task id")
		}
		self.task_queue.push(task_id).expect("Too many tasks!")
	}

	fn run_next_tasks(&mut self) {
		let Self {
			tasks,
			task_queue,
			waker_cache,
		} = self;

		while let Some(task_id) = task_queue.pop() {
			let task = match tasks.get_mut(&task_id) {
				Some(task) => task,
				None => continue,
			};

			let waker = waker_cache
				.entry(task_id)
				.or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

			use core::task::{Poll, Context};

			let mut ctx = Context::from_waker(waker);
			match task.poll(&mut ctx) {
				Poll::Pending => {},
				Poll::Ready(()) => {
					tasks.remove(&task_id);
					waker_cache.remove(&task_id);
				}
			}
		}
	}

	// Takes ownership. Doesn't matter cuz it doesn't return.
	pub fn run(mut self) -> ! {
		loop {
			self.run_next_tasks();
			self.sleep_if_idle();
		}
	}

	fn sleep_if_idle(&self) {
		use x86_64::instructions::interrupts::{self, enable_and_hlt};

		interrupts::disable();
		if self.task_queue.is_empty() {
			enable_and_hlt();
		} else {
			interrupts::enable();
		}
	}
}

struct TaskWaker {
	task_id: TaskId,
	task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
	fn wake_task(&self) {
		self.task_queue.push(self.task_id).expect("Too many tasks!");
	}

	fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
		Waker::from(Arc::new(TaskWaker {
			task_id, task_queue
		}))
	}
}

impl Wake for TaskWaker {
	fn wake(self: Arc<Self>) {
		self.wake_task();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.wake_task();
	}
}
