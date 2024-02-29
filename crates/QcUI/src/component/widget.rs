use egui::{Color32, Margin};
use serde::{Deserialize, Serialize};
use thunderdome::Index;

#[derive(Serialize, Deserialize, Debug)]
pub struct Widget {
    pub id: Index,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub margin: Margin,
    pub padding: Margin,
    pub background: Color32,
    pub foreground: Color32,
    pub visible: bool,
    pub children: Vec<Index>,
    pub parent: Index,
    pub z_index: u32,
    pub enabled: bool,
    pub opacity: f32,
    pub font_size: f32,
}

impl Default for Widget {
    fn default() -> Self {
        Self {
            id: Index::DANGLING,
            name: "Widget".to_string(),
            width: 100.0,
            height: 100.0,
            margin: Default::default(),
            padding: Default::default(),
            background: Color32::WHITE,
            foreground: Color32::BLACK,
            visible: true,
            children: Vec::new(),
            parent: Index::DANGLING,
            z_index: 1,
            enabled: true,
            opacity: 1.0,
            font_size: 14.,
        }
    }
}

impl Widget {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }


    pub fn with_background(mut self, background: Color32) -> Self {
        self.background = background;
        self
    }

    pub fn with_foreground(mut self, foreground: Color32) -> Self {
        self.foreground = foreground;
        self
    }

    
}
