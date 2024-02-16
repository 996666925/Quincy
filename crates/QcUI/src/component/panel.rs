use std::sync::mpsc::Sender;

use egui::{
    plot::Orientation, style::Spacing, Color32, Frame, Layout, Margin, RichText, Ui, Vec2,
    WidgetText,
};
use serde::{Deserialize, Serialize};
use thunderdome::Arena;

use taffy::style::FlexDirection;

use QcMacros::Control;

use QcTools::message::messageSender::MessageSender;
use QcWindowing::{CursorIcon, Window};

use crate::message::UiMessage;

use super::{UiNode, UiNodeTrait};

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Panel {
    width: f32,
    height: f32,
    orientation: FlexDirection,
    children: Arena<UiNode>,
    margin: Margin,
    padding: Margin,
    background: Color32,
    id: Index,
    spacing: f32,
}

#[typetag::serde]
impl UiNodeTrait for Panel {
    fn renderFrame(&self, ui: &mut egui::Ui) -> egui::Frame {
        let frame = Frame::none()
            .fill(self.background)
            .inner_margin(self.padding)
            .outer_margin(self.margin);

        frame
    }

    fn renderInner(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>) {
        ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing = Vec2::new(self.spacing, self.spacing);

            ui.set_width(self.width);
            ui.set_height(self.height);
            match self.orientation {
                FlexDirection::Column => {
                    ui.vertical(|ui| {
                        for (_, node) in self.children.iter_mut() {
                            node.value.render(ui, sender);
                        }
                    });
                }
                FlexDirection::Row => {
                    ui.horizontal(|ui| {
                        for (_, node) in self.children.iter_mut() {
                            node.value.render(ui, sender);
                        }
                    });
                }
                _ => {}
            }
        });
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            height: 0.,
            width: 0.,
            orientation: FlexDirection::Row,
            children: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            background: Default::default(),
            id: Index::DANGLING,
            spacing: 0.,
        }
    }
}

impl Panel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn orientation(mut self, orientation: FlexDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }

    pub fn background(mut self, background: Color32) -> Self {
        self.background = background;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn addChild(&mut self, node: UiNode) -> Index {
        let index = self.children.insert(node);
        self.children[index].value.setId(index);
        index
    }

    pub fn removeChild(&mut self, node: Index) -> Option<UiNode> {
        self.children.remove(node)
    }
}
