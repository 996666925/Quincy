use egui::{Color32, Margin, Rounding, Stroke};
use serde::{Deserialize, Deserializer, Serialize};
use thunderdome::Index;
use uuid::Uuid;

use crate::message::{EventBuilder, UiMessageType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Widget {
    pub uuid: Uuid,
    pub id: Index,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub margin: Margin,
    pub padding: Margin,
    pub background: Color32,
    pub foreground: Color32,
    pub radius: Rounding,
    pub border: Stroke,
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
            uuid: Uuid::new_v4(),
            id: Index::DANGLING,
            name: "Widget".to_string(),
            width: 0.0,
            height: 0.0,
            margin: Default::default(),
            padding: Default::default(),
            radius: Default::default(),
            border: Default::default(),
            background: Color32::TRANSPARENT,
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

    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_radius(mut self, radius: Rounding) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_border(mut self, border: Stroke) -> Self {
        self.border = border;
        self
    }

    pub fn on_event(
        self,
        event: UiMessageType,
        func: Box<dyn Fn(UiMessageType)>,
    ) -> EventBuilder {
        EventBuilder::new(self).on_event(event, func)
    }


}
