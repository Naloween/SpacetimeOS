pub mod executor;
pub mod keyboard;

use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

pub struct Task {
    pub id: u64,
    pub module_id: u64,
    pub reducer_id: u64,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(
        id: u64,
        module_id: u64,
        reducer_id: u64,
        future: impl Future<Output = ()> + 'static,
    ) -> Self {
        Task {
            id,
            module_id,
            reducer_id,
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
