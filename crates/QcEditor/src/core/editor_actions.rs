use std::cell::Cell;

use thunderdome::Index;

#[derive(Debug)]
pub struct EditorActions {
    pub target: Cell<Option<Index>>,
}

impl EditorActions {
    pub fn new() -> Self {
        Self {
            target: Cell::new(None),
        }
    }
    pub fn select(&self, index: Option<Index>) {
        self.target.set(index);
    }

    pub fn current(&self) -> Option<Index> {
        self.target.get()
    }
}
