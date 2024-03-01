use std::sync::mpsc::Sender;

use egui::{Color32, Frame, Margin};
use serde::{Deserialize, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::Window;

use crate::{core::context::UiContext, message::UiMessage};

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Label {
    text: String,
    widget: Widget,
}

#[typetag::serde]
impl UiNodeTrait for Label {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = Frame::none()
            .inner_margin(self.padding)
            .outer_margin(self.margin);
        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        let UiContext { ui, sender } = ctx;
        ui.scope(|ui| {
            ui.visuals_mut().override_text_color = Some(self.foreground);
            ui.style_mut().wrap = Some(false);
            let label = egui::Label::new(&self.text);

            ui.add(label);
        });
    }
}

impl Label {
    pub fn new(widget: Widget) -> Self {
        Self {
            text: String::new(),
            widget: widget,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
