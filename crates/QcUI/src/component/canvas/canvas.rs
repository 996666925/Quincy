use egui::Ui;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, Index as IndexOps, IndexMut},
};
use thunderdome::{Arena, Index};
use QcMacros::UiComp;

use crate::{core::uiBind::UiBind, message::UiMessageType};

use super::UiNode;

#[derive(Debug, UiComp, Deserialize, Serialize)]
pub struct Canvas {
    pub pool: Arena<UiNode>,
    pub uiBindList: HashMap<Index, Vec<UiBind>>,
}

impl Deref for Canvas {
    type Target = Arena<UiNode>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

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
            pool: Default::default(),
            uiBindList: Default::default(),
        }
    }

    pub fn addUiBind(&mut self, comp: Index, bind: UiBind) {
        self.uiBindList
            .entry(comp)
            .and_modify(|vec| vec.push(bind.clone()))
            .or_insert(vec![bind.clone()]);
    }

    pub fn getUiBind(&mut self, comp: Index) -> Option<&Vec<UiBind>> {

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
}
