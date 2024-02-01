use std::{cell::{Cell, RefCell}, borrow::BorrowMut};

use rust_embed::{EmbeddedFile, RustEmbed};
use OvRender::resources::Resource;

pub trait ResourceTrait {
    fn get(&self, file_path: &str) -> Resource;
}

impl<T> ResourceTrait for T
where
    T: RustEmbed,
{
    fn get(&self, file_path: &str) -> Resource {
        Resource {
            name: file_path.to_string(),
            file: Self::get(file_path).unwrap(),
        }
    }
}

pub struct ResourceManager {
    value: RefCell<Option<Box<dyn ResourceTrait>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            value: RefCell::new(None),
        }
    }

    pub fn setPath(&self, value: Box<dyn ResourceTrait + 'static>) {
        self.value.replace(Some(value));
    }

    pub fn get(&self, name: &str) -> Option<Resource> {
        let res = &*self.value.borrow();
        
        if let Some(res) = res {
            Some(res.get(name))
        } else {
            None
        }
    }
}
