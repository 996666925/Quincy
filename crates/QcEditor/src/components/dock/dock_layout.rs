use egui_tiles::{Container, Grid, Linear, Tabs, TileId, Tiles};
use serde::{Deserialize, Serialize};

use super::DockItem;

#[derive(Serialize, Deserialize, Debug)]
pub enum DockLayoutType {
    Tabs,
    Linear,
    Grid,
}

impl Default for DockLayoutType {
    fn default() -> Self {
        DockLayoutType::Linear
    }
}

#[derive(Debug)]
pub struct DockLayout {
    pub container: DockLayoutType,
    pub children: Vec<DockItem>,
    pub share: f32,
}

impl Default for DockLayout {
    fn default() -> Self {
        Self {
            container: DockLayoutType::Linear,
            children: vec![],
            share: 0.,
        }
    }
}

impl DockLayout {
    pub fn new(container: DockLayoutType) -> Self {
        Self {
            container,
            children: vec![],
            share: 0.,
        }
    }

    pub fn with_children(mut self, children: Vec<DockItem>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self, tiles: &mut Tiles<DockItem>) -> TileId {
        let mut children = vec![];

        let children = match self.container {
            DockLayoutType::Tabs => {
                let mut container = Tabs {
                    children,
                    ..Default::default()
                };
                for child in self.children {
                    let id = tiles.insert_pane(child);
                    container.children.push(id);
                }
                Container::Tabs(container)
            }
            DockLayoutType::Linear => {
                let mut container = Linear {
                    dir: egui_tiles::LinearDir::Vertical,
                    children,
                    ..Default::default()
                };

                for child in self.children {
                    let share = child.share;
                    let id = tiles.insert_pane(child);

                    container.children.push(id);
                    if share != 0. {
                        container.shares.set_share(id, share);
                    }
                }
                Container::Linear(container)
            }
            DockLayoutType::Grid => todo!(),
        };

        tiles.insert_container(children)
    }

    pub fn with_share(mut self, share: f32) -> Self {
        self.share = share;
        self
    }
}
