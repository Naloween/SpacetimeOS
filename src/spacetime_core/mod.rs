use alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc};
use core::{any::Any, pin::Pin};
use crossbeam_queue::ArrayQueue;
use spin::Mutex;

use crate::{
    spacetime_core::module::{AccessLevel, Module},
    spacetime_core::reducer::{Reducer, ReducerContext},
    task::{Task, executor::Executor},
};

pub mod module;
pub mod reducer;

#[derive(Clone)]
pub struct SpacetimeCore {
    pub modules: Arc<Mutex<BTreeMap<u64, Module>>>,
    pub tasks: Arc<Mutex<BTreeMap<u64, Task>>>,
    pub task_queue: Arc<ArrayQueue<u64>>,
    next_task_id: Arc<Mutex<u64>>,
    next_module_id: Arc<Mutex<u64>>,
    executor: Arc<Mutex<Executor>>,
}

impl SpacetimeCore {
    pub fn new() -> SpacetimeCore {
        let executor = Executor::new();
        SpacetimeCore {
            modules: Arc::new(Mutex::new(BTreeMap::new())),
            tasks: executor.tasks.clone(),
            task_queue: executor.task_queue.clone(),
            next_module_id: Arc::new(Mutex::new(0)),
            next_task_id: Arc::new(Mutex::new(0)),
            executor: Arc::new(Mutex::new(executor)),
        }
    }

    pub fn run(&mut self) {
        self.executor.lock().run();
    }

    pub fn insert_module(&mut self, name: String, access_level: AccessLevel) -> u64 {
        let mut next_module_id_guard = self.next_module_id.lock();
        let module_id = *next_module_id_guard;
        *next_module_id_guard += 1;
        let module = Module::new(module_id, name, access_level);

        self.modules.lock().insert(module.id, module);

        return module_id;
    }

    pub fn delete_module(&mut self, module_id: u64) {
        self.modules.lock().remove(&module_id);
    }

    pub fn insert_reducer(
        &mut self,
        module_id: u64,
        name: String,
        job: Box<dyn Fn(ReducerContext) -> Pin<Box<dyn Future<Output = ()>>>>,
    ) -> Result<u64, &str> {
        if let Some(module) = self.modules.lock().get_mut(&module_id) as Option<&mut Module> {
            Ok(module.insert_reducer(name, job))
        } else {
            Err("Module not found")
        }
    }

    pub fn delete_reducer(&mut self, module_id: u64, reducer_id: u64) {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            module.delete_reducer(reducer_id);
        }
    }

    pub fn call_reducer(&mut self, module_id: u64, reducer_id: u64) {
        if let Some(module) = self.modules.lock().get(&module_id) as Option<&Module> {
            if let Some(reducer) = module.reducers.get(&reducer_id) as Option<&Reducer> {
                let ctx =
                    ReducerContext::new(self.clone(), module_id, reducer_id, module.access_level);
                let mut next_task_id_guard = self.next_task_id.lock();
                let task = Task::new(
                    *next_task_id_guard,
                    module_id,
                    reducer_id,
                    (reducer.job)(ctx),
                );
                *next_task_id_guard += 1;

                self.task_queue.push(task.id).expect("Couldn't add task");
                self.tasks.lock().insert(task.id, task);
            }
        }
    }

    pub fn insert_table<T: 'static + Clone>(
        &mut self,
        module_id: u64,
        name: String,
    ) -> Result<u64, &str> {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            Ok(module.insert_table::<T>(name))
        } else {
            Err("Module not found")
        }
    }

    pub fn delete_table(&mut self, module_id: u64, table_id: u64) {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            module.delete_table(table_id);
        }
    }

    pub fn insert_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        content: Box<dyn Any>,
    ) -> Result<u64, &str> {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            module.insert_table_row(table_id, content)
        } else {
            Err("Module not found")
        }
    }

    pub fn delete_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
    ) -> Result<Box<(dyn Any + 'static)>, &str> {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            module.delete_table_row(table_id, row_id)
        } else {
            Err("Module not found")
        }
    }

    pub fn update_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
        content: Box<dyn Any>,
    ) -> Result<Box<dyn Any>, &str> {
        if let Some(module) = self.modules.lock().get_mut(&module_id) {
            module.update_table_row(table_id, row_id, content)
        } else {
            Err("Module not found")
        }
    }

    pub fn get_table_row(
        &self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
    ) -> Result<Box<dyn Any>, &str> {
        if let Some(module) = self.modules.lock().get(&module_id) {
            module.get_table_row(table_id, row_id)
        } else {
            Err("Module not found")
        }
    }
}
