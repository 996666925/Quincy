use std::ops::{Deref, DerefMut};

use egui_tiles::{Container, Tabs, Tile, TileId, Tiles};
use serde::{Deserialize, Serialize};
use QcUI::component::UiNode;

use crate::panel::NavPanel;

use super::DockView;

#[derive(Debug)]
pub struct DockItem {
    pub name: String,
    pub child: Box<dyn DockView>,
    pub share: f32,
    pub show_tab: bool,
}

impl Deref for DockItem {
    type Target = Box<dyn DockView>;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl DerefMut for DockItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

impl DockItem {
    pub fn new(name: &str, child: Box<dyn DockView>) -> Self {
        Self {
            name: name.to_string(),
            child,
            share: 0.,
            show_tab: true,
        }
    }

    pub fn with_share(mut self, share: f32) -> Self {
        self.share = share;
        self
    }

    /// 是否显示标题
    pub fn with_show_tab(mut self, show: bool) -> Self {
        self.show_tab = show;
        self
    }

    pub fn build(self, tiles: &mut Tiles<DockItem>) -> TileId {
        let id = tiles.insert_pane(self);
        id
    }
}
