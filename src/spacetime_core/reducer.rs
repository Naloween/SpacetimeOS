use crate::{
    spacetime_core::SpacetimeCore,
    spacetime_core::module::{AccessLevel, ModuleInfos},
};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{any::Any, pin::Pin};

pub struct Reducer {
    pub id: u64,
    pub module_id: u64,
    pub name: String,
    pub job: Box<dyn Fn(ReducerContext) -> Pin<Box<dyn Future<Output = ()>>>>,
}

impl Reducer {
    pub fn get_infos(&self) -> ReducerInfos {
        ReducerInfos {
            id: self.id,
            module_id: self.module_id,
            name: self.name.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReducerInfos {
    pub id: u64,
    pub module_id: u64,
    pub name: String,
}

pub struct ReducerContext {
    spacetime_core: SpacetimeCore,
    access_level: AccessLevel,
    pub reducer_id: u64,
    pub module_id: u64,
}

impl ReducerContext {
    pub fn new(
        engine: SpacetimeCore,
        module_id: u64,
        reducer_id: u64,
        access_level: AccessLevel,
    ) -> Self {
        Self {
            spacetime_core: engine,
            access_level,
            module_id,
            reducer_id,
        }
    }

    pub fn get_modules_id(&self) -> Result<Vec<u64>, &str> {
        if self.access_level == AccessLevel::Admin {
            Ok(self.spacetime_core.modules.lock().keys().cloned().collect())
        } else {
            Err("You do not have access")
        }
    }

    pub fn get_module_infos(&self, module_id: u64) -> Result<ModuleInfos, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            if let Some(module) = self.spacetime_core.modules.lock().get(&module_id) {
                Ok(module.get_infos())
            } else {
                Err("Module not found")
            }
        } else {
            Err("You do not have access")
        }
    }

    pub fn call_reducer(&mut self, module_id: u64, reducer_id: u64) {
        self.spacetime_core.call_reducer(module_id, reducer_id);
    }

    pub fn insert_module(&mut self, name: String, access_level: AccessLevel) -> Result<(), &str> {
        if self.access_level == AccessLevel::Admin {
            self.spacetime_core.insert_module(name, access_level);
            Ok(())
        } else {
            Err("You do not have access")
        }
    }

    pub fn insert_reducer(
        &mut self,
        module_id: u64,
        name: String,
        job: Box<dyn Fn(ReducerContext) -> Pin<Box<dyn Future<Output = ()>>>>,
    ) -> Result<u64, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core.insert_reducer(module_id, name, job)
        } else {
            Err("You do not have access")
        }
    }

    pub fn insert_table<T: 'static + Clone>(
        &mut self,
        module_id: u64,
        name: String,
    ) -> Result<u64, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core.insert_table::<T>(module_id, name)
        } else {
            Err("You do not have access")
        }
    }

    pub fn insert_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        content: Box<dyn Any>,
    ) -> Result<u64, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core
                .insert_table_row(module_id, table_id, content)
        } else {
            Err("You do not have access")
        }
    }

    pub fn delete_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
    ) -> Result<Box<dyn Any>, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core
                .delete_table_row(module_id, table_id, row_id)
        } else {
            Err("You do not have access")
        }
    }

    pub fn update_table_row(
        &mut self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
        content: Box<dyn Any>,
    ) -> Result<Box<dyn Any>, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core
                .update_table_row(module_id, table_id, row_id, content)
        } else {
            Err("You do not have access")
        }
    }

    pub fn get_table_row(
        &self,
        module_id: u64,
        table_id: u64,
        row_id: u64,
    ) -> Result<Box<dyn Any>, &str> {
        if module_id == self.module_id || self.access_level == AccessLevel::Admin {
            self.spacetime_core
                .get_table_row(module_id, table_id, row_id)
        } else {
            Err("You do not have access")
        }
    }
}
