use egui::Ui;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Index as IndexOps, IndexMut},
};
use thunderdome::{
    iter::{Iter, IterMut},
    Arena, Index,
};
use uuid::Uuid;
use QcMacros::{Component, UiComp};

use crate::{core::uiBind::UiBind, message::UiMessageType};

use super::UiNode;

#[derive(Debug, UiComp, Deserialize, Serialize)]
pub struct Canvas {
    inner: ComponentInner,
    pub pool: Arena<UiNode>,
    pub uiBindList: HashMap<Uuid, Vec<UiBind>>,
}

// impl Deref for Canvas {
//     type Target = Arena<UiNode>;

//     fn deref(&self) -> &Self::Target {
//         &self.pool
//     }
// }

// impl DerefMut for Canvas {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.pool
//     }
// }

impl IndexOps<Index> for Canvas {
    type Output = UiNode;

    fn index(&self, index: Index) -> &Self::Output {
        &self.pool[index]
    }
}
impl IndexMut<Index> for Canvas {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.pool[index]
    }
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            inner: ComponentInner::default(),
            pool: Default::default(),
            uiBindList: Default::default(),
        }
    }

    pub fn addUiBind(&mut self, comp: Uuid, bind: UiBind) {
        self.uiBindList.entry(comp).or_insert(vec![]).push(bind);
    }

    pub fn add_ui_bind_list(&mut self, comp: Uuid, mut bind: Vec<UiBind>) {
        self.uiBindList.entry(comp).or_insert(vec![]).append(&mut bind);
    }

    pub fn getUiBind(&self, comp: Uuid) -> Option<&Vec<UiBind>> {
        self.uiBindList.get(&comp)
    }

    pub fn addChild(&mut self, node: UiNode) -> Index {
        let index = self.pool.insert(node);
        self[index].value.setId(index);
        index
    }

    pub fn removeChild(&mut self, node: Index) -> Option<UiNode> {
        self.pool.remove(node)
    }

    pub fn iter(&self) -> Iter<'_, UiNode> {
        self.pool.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, UiNode> {
        self.pool.iter_mut()
    }
}
