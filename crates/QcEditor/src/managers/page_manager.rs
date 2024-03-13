use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, Index as IndexOps, IndexMut},
};

use thunderdome::{Arena, Index};
use QcUI::component::PanelWindow;

use crate::core::message::Page;

type PageValue = Box<dyn PanelWindow>;
pub struct PageManager {
    pages: HashMap<Page, PageValue>,
    current: Page,
}

impl Deref for PageManager {
    type Target = HashMap<Page, PageValue>;

    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}

impl DerefMut for PageManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pages
    }
}

impl PageManager {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            current: Page::ProjectHub,
        }
    }

    pub fn get_current(&self) -> Option<&PageValue> {
        self.pages.get(&self.current)
    }

    pub fn get_current_mut(&mut self) -> Option<&mut PageValue> {
        self.pages.get_mut(&self.current)
    }

    pub fn add_page(&mut self, key: Page, page: PageValue) {
        let set_current=self.pages.is_empty();
        self.entry(key).or_insert(page);
        if set_current{
            self.current = key;
        }
    
    }

    pub fn navigate_to(&mut self, key: Page) {
        if let Some(page)=self.pages.get(&key){
            self.current = key;

        }
      
    }
}
