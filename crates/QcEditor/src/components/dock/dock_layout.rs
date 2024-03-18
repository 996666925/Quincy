use egui_tiles::{Container, Grid, Linear, LinearDir, Tabs, TileId, Tiles};
use serde::{Deserialize, Serialize};

use super::{DockItem, DockWidget};

#[derive(Serialize, Deserialize, Debug)]
pub enum DockContainer {
    Tabs,
    Linear(LinearDir),
    Grid,
}

impl Default for DockContainer {
    fn default() -> Self {
        DockContainer::Linear(LinearDir::Horizontal)
    }
}

impl DockContainer {
    pub fn linear_vertical() -> Self {
        DockContainer::Linear(LinearDir::Vertical)
    }
}

#[derive(Debug)]
pub struct DockLayout {
    pub container: DockContainer,
    pub children: Vec<DockWidget>,
    pub share: f32,
}

impl Default for DockLayout {
    fn default() -> Self {
        Self {
            container: DockContainer::Linear(LinearDir::Horizontal),
            children: vec![],
            share: 0.,
        }
    }
}

impl DockLayout {
    pub fn new(container: DockContainer) -> Self {
        Self {
            container,
            children: vec![],
            share: 0.,
        }
    }

    pub fn with_container(mut self, container: DockContainer) -> Self {
        self.container = container;
        self
    }

    pub fn with_children(mut self, children: Vec<DockWidget>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self, tiles: &mut Tiles<DockItem>) -> TileId {
        let children = vec![];

        let children = match self.container {
            DockContainer::Tabs => {
                let mut container = Tabs {
                    children,
                    ..Default::default()
                };
                for child in self.children {
                    let id = child.build(tiles);
                    container.children.push(id);
                }
                Container::Tabs(container)
            }
            DockContainer::Linear(dir) => {
                let mut container = Linear {
                    dir,
                    children,
                    ..Default::default()
                };

                for child in self.children {
                    let share = child.get_share();
                    let id = child.build(tiles);

                    container.children.push(id);
                    if share != 0. {
                        container.shares.set_share(id, share);
                    }
                }
                Container::Linear(container)
            }
            DockContainer::Grid => todo!(),
        };

        tiles.insert_container(children)
    }

    pub fn with_share(mut self, share: f32) -> Self {
        self.share = share;
        self
    }
}
