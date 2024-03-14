use std::sync::mpsc::Sender;

use egui::{style::Spacing, Frame, Layout, Ui, Vec2};
use serde::{Deserialize, Serialize};
use thunderdome::Arena;

use taffy::style::FlexDirection;

use QcCore::ecs::graph::Graph;
use QcMacros::Control;

use QcTools::message::messageSender::MessageSender;
use QcWindowing::{CursorIcon, Window};

use crate::{core::context::UiContext, message::UiMessage};

use super::{Canvas, UiNode, UiNodeTrait};


#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Grid {
    pub widget: Widget,
    pub children: Arena<UiNode>,
    pub columns: i32,
    pub spacing: Vec2,
}

#[typetag::serde]
impl UiNodeTrait for Grid {
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
            if self.width != 0.0 && self.height != 0.0 {
                ui.set_width(self.width);
                ui.set_height(self.height);
            }

            let res = egui::Grid::new(&self.name)
                .spacing(self.spacing)
                .show(ui, |ui| {
                    let mut index = 1;
                    for (_, node) in self.children.iter_mut() {
                        node.value.render(&mut UiContext::new(ui, sender));

                        if index % self.columns == 0 {
                            ui.end_row();
                        }
                        index += 1;
                    }
                });

            let rect = res.response.rect;
            self.width = rect.width();
            self.height = rect.height();
        });
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            widget: Default::default(),
            children: Default::default(),
            columns: 1,
            spacing: Default::default(),
        }
    }
}

impl Grid {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            columns: 1,
            ..Default::default()
        }
    }

    pub fn with_spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_columns(mut self, columns: i32) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_children(mut self, children: Vec<UiNode>) -> Self {
        self.children = Arena::new();
        for child in children {
            self.addChild(child);
        }
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

    pub fn build(self, canvas: &mut Canvas) -> Index {
        canvas.addChild(UiNode::new(self))
    }
}
