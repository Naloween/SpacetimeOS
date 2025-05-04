use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use core::{
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::task::executor::{Executor, Spawner};

pub struct SpacetimeCore {
    users: BTreeMap<u64, User>,
    modules: BTreeMap<u64, Module>,
    executor: Executor,
    spawner: Spawner,
}

impl SpacetimeCore {
    pub fn new() -> SpacetimeCore {
        let executor = Executor::new();
        SpacetimeCore {
            users: BTreeMap::new(),
            modules: BTreeMap::new(),
            spawner: Spawner::new(&executor),
            executor,
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    pub fn delete_user(&mut self, user_id: &u64) -> Option<User> {
        self.users.remove(user_id)
    }

    pub fn delete_module(&mut self, module_id: &u64) -> Option<Module> {
        self.modules.remove(module_id)
    }

    pub fn publish_module(&mut self, module: Module) {
        self.modules.insert(module.id, module);
    }

    pub fn run(&mut self) {
        self.executor.run();
    }
}

pub struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(name: String) -> User {
        static NEXT_USER_ID: AtomicU64 = AtomicU64::new(0);
        User {
            id: NEXT_USER_ID.fetch_add(1, Ordering::Relaxed),
            name,
        }
    }
}

pub struct Table {
    name: String,
}

pub struct Reducer {
    name: String,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

pub struct Module {
    id: u64,
    name: String,
    tables: BTreeMap<u64, Table>,
    reducers: BTreeMap<u64, Reducer>,
    next_table_id: u64,
    next_reducer_id: u64,
}

impl Module {
    pub fn new(name: String) -> Module {
        static NEXT_MODULE_ID: AtomicU64 = AtomicU64::new(0);
        Module {
            id: NEXT_MODULE_ID.fetch_add(1, Ordering::Relaxed),
            name,
            tables: BTreeMap::new(),
            reducers: BTreeMap::new(),
            next_reducer_id: 0,
            next_table_id: 0,
        }
    }
}
