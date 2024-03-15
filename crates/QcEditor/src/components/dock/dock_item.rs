use serde::{Deserialize, Serialize};
use QcUI::component::UiNode;

use super::DockView;

#[derive( Debug)]
pub struct DockItem {
    pub name: String,
    pub child: Box<dyn DockView>,
    pub share: f32,
}

impl DockItem {
    pub fn new(name: &str, child: Box<dyn DockView>) -> Self {
        Self {
            name: name.to_string(),
            child,
            share: 0.,
        }
    }

    pub fn with_share(mut self, share: f32) -> Self {
        self.share = share;
        self
    }
}
