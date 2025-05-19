use core::any::Any;

use alloc::{boxed::Box, collections::BTreeMap, string::String};

pub trait TableBase {
    fn insert_row_any(&mut self, content: Box<dyn Any>) -> u64;

    fn delete_row_any(&mut self, row_id: u64) -> Option<Box<dyn Any>>;

    fn update_row_any(&mut self, row_id: u64, content: Box<dyn Any>) -> Option<Box<dyn Any>>;

    fn get_row_any(&self, row_id: u64) -> Option<Box<dyn Any>>;

    fn get_infos(&self) -> TableInfos;
}

pub struct Table<T> {
    pub id: u64,
    pub name: String,
    rows: BTreeMap<u64, Box<T>>,
    next_row_id: u64,
}

impl<T: Clone> Table<T> {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            rows: BTreeMap::new(),
            next_row_id: 0,
        }
    }

    fn insert_row(&mut self, content: Box<T>) -> u64 {
        let row_id = self.next_row_id;
        self.next_row_id += 1;
        self.rows.insert(row_id, content);
        return row_id;
    }

    fn delete_row(&mut self, row_id: u64) -> Option<Box<T>> {
        self.rows.remove(&row_id)
    }

    fn update_row(&mut self, row_id: u64, content: Box<T>) -> Option<Box<T>> {
        self.rows.insert(row_id, content)
    }

    fn get_row(&self, row_id: u64) -> Option<Box<T>> {
        let row = self.rows.get(&row_id);
        if let Some(row) = row {
            let row = (*row).clone();
            return Some(row);
        }
        return None;
    }
}

impl<T: 'static + Clone> TableBase for Table<T> {
    fn insert_row_any(&mut self, content: Box<dyn Any>) -> u64 {
        let content = content.downcast::<T>().expect("Wrong type !");
        return self.insert_row(content);
    }

    fn delete_row_any(&mut self, row_id: u64) -> Option<Box<dyn Any + 'static>> {
        if let Some(old_row) = self.delete_row(row_id) {
            let old_row = old_row as Box<dyn Any>;
            return Some(old_row);
        }
        return None;
    }

    fn update_row_any(&mut self, row_id: u64, content: Box<dyn Any>) -> Option<Box<dyn Any>> {
        let content = content.downcast::<T>().expect("Wrong type !");
        if let Some(old_row) = self.update_row(row_id, content) {
            let old_row = old_row as Box<dyn Any>;
            return Some(old_row);
        }
        return None;
    }

    fn get_row_any(&self, row_id: u64) -> Option<Box<dyn Any>> {
        if let Some(row) = self.get_row(row_id) {
            let row = row as Box<dyn Any>;
            return Some(row);
        }
        return None;
    }

    fn get_infos(&self) -> TableInfos {
        TableInfos {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableInfos {
    pub id: u64,
    pub name: String,
}
