use super::Task;
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use spin::Mutex;

const MAX_NUMBER_TASKS: usize = 100;

pub struct Executor {
    pub tasks: Arc<Mutex<BTreeMap<u64, Task>>>,
    pub task_queue: Arc<ArrayQueue<u64>>,
    waker_cache: BTreeMap<u64, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: Arc::new(Mutex::new(BTreeMap::new())),
            task_queue: Arc::new(ArrayQueue::new(MAX_NUMBER_TASKS)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let mut tasks_guard = tasks.lock();
            let mut task = match tasks_guard.remove(&task_id) {
                Some(task) => task,
                // task no longer exists
                None => {
                    drop(tasks_guard);
                    continue;
                }
            };
            drop(tasks_guard);

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {
                    // Re-insert the task
                    let mut tasks_guard = tasks.lock();
                    if tasks_guard.insert(task_id, task).is_some() {
                        panic!("Task {:?} re-inserted while being polled", task_id);
                    }
                }
            }
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
    task_id: u64,
    task_queue: Arc<ArrayQueue<u64>>,
}

impl TaskWaker {
    fn new(task_id: u64, task_queue: Arc<ArrayQueue<u64>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
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
