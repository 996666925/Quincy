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

use crate::{core::context::UiContext, message::UiMessage};

use super::{UiNode, UiNodeTrait};

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Panel {
    pub widget: Widget,
    pub orientation: FlexDirection,
    pub children: Arena<UiNode>,
    pub spacing: f32,
}

#[typetag::serde]
impl UiNodeTrait for Panel {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = Frame::none()
            .fill(self.background)
            .inner_margin(self.padding)
            .outer_margin(self.margin);

        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        let UiContext { ui, sender } = ctx;
        ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing = Vec2::new(self.spacing, self.spacing);

            if self.width != 0.0 && self.height != 0.0 {
                ui.set_width(self.width);
                ui.set_height(self.height);
            }

            let res = match self.orientation {
                FlexDirection::Column => ui.vertical(|ui| {
                    for (_, node) in self.children.iter_mut() {
                        node.value.render(&mut UiContext::new(ui, sender));
                    }
                }),
                FlexDirection::Row => ui.horizontal(|ui: &mut Ui| {
                    for (_, node) in self.children.iter_mut() {
                        node.value.render(&mut UiContext::new(ui, sender));
                    }
                }),
                _ => {
                    panic!("只支持Row和Column")
                }
            };

            let rect = res.response.rect;
            self.width = rect.width();
            self.height = rect.height();
        });
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            widget: Default::default(),
            orientation: FlexDirection::Row,
            children: Default::default(),
            spacing: 0.,
        }
    }
}

impl Panel {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            ..Default::default()
        }
    }

    pub fn with_orientation(mut self, orientation: FlexDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
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
