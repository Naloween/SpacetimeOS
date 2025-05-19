use core::{any::Any, pin::Pin};

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use crate::{
    reducer::{Reducer, ReducerContext, ReducerInfos},
    table::{Table, TableBase, TableInfos},
};

pub struct Module {
    pub id: u64,
    pub name: String,
    pub reducers: BTreeMap<u64, Reducer>,
    pub access_level: AccessLevel,
    tables: BTreeMap<u64, Box<dyn TableBase>>,
    next_reducer_id: u64,
    next_table_id: u64,
}

impl Module {
    pub fn new(id: u64, name: String, access_level: AccessLevel) -> Self {
        Module {
            id,
            name,
            access_level,
            reducers: BTreeMap::new(),
            tables: BTreeMap::new(),
            next_reducer_id: 0,
            next_table_id: 0,
        }
    }

    pub fn insert_reducer(
        &mut self,
        name: String,
        job: Box<dyn Fn(ReducerContext) -> Pin<Box<dyn Future<Output = ()>>>>,
    ) -> u64 {
        let reducer = Reducer {
            id: self.next_reducer_id,
            module_id: self.id,
            name,
            job,
        };
        self.next_reducer_id += 1;
        self.reducers.insert(reducer.id, reducer);

        return self.next_reducer_id - 1;
    }

    pub fn delete_reducer(&mut self, reducer_id: u64) {
        self.reducers.remove(&reducer_id);
    }

    pub fn insert_table<T: 'static + Clone>(&mut self, name: String) -> u64 {
        let table_id = self.next_table_id;
        let table = Table::<T>::new(table_id, name);
        self.next_table_id += 1;
        self.tables.insert(table.id, Box::new(table));
        return table_id;
    }

    pub fn delete_table(&mut self, table_id: u64) {
        self.tables.remove(&table_id);
    }

    pub fn insert_table_row(
        &mut self,
        table_id: u64,
        content: Box<dyn Any>,
    ) -> Result<u64, &'static str> {
        let table = self.tables.get_mut(&table_id);
        if let Some(table) = table {
            Ok(table.insert_row_any(content))
        } else {
            Err("Table not found")
        }
    }

    pub fn delete_table_row(
        &mut self,
        table_id: u64,
        row_id: u64,
    ) -> Result<Box<(dyn Any + 'static)>, &'static str> {
        let table = self.tables.get_mut(&table_id);
        if let Some(table) = table {
            if let Some(row) = table.delete_row_any(row_id) {
                Ok(row)
            } else {
                Err("Row not found")
            }
        } else {
            Err("Table not found")
        }
    }

    pub fn update_table_row(
        &mut self,
        table_id: u64,
        row_id: u64,
        content: Box<dyn Any>,
    ) -> Result<Box<dyn Any>, &'static str> {
        let table = self.tables.get_mut(&table_id);
        if let Some(table) = table {
            if let Some(row) = table.update_row_any(row_id, content) {
                Ok(row)
            } else {
                Err("Row not found")
            }
        } else {
            Err("Table not found")
        }
    }

    pub fn get_table_row(&self, table_id: u64, row_id: u64) -> Result<Box<dyn Any>, &'static str> {
        let table = self.tables.get(&table_id);
        if let Some(table) = table {
            if let Some(row) = table.get_row_any(row_id) {
                Ok(row)
            } else {
                Err("Row not found")
            }
        } else {
            Err("Table not found")
        }
    }

    pub fn get_infos(&self) -> ModuleInfos {
        ModuleInfos {
            id: self.id,
            name: self.name.clone(),
            access_level: self.access_level,
            reducers: self
                .reducers
                .values()
                .map(|reducer| reducer.get_infos())
                .collect(),
            tables: self
                .tables
                .values()
                .map(|table| table.get_infos())
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleInfos {
    pub id: u64,
    pub name: String,
    pub reducers: Vec<ReducerInfos>,
    pub tables: Vec<TableInfos>,
    pub access_level: AccessLevel,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    Admin,
    Standard,
}
