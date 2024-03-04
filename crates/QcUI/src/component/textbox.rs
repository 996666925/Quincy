use std::fmt::Debug;

use egui::{text_edit::TextEditState, Align, Align2, Color32, Stroke, TextBuffer};
use serde::{Deserialize, Serialize};
use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::{dpi::LogicalPosition, Window};

use crate::{
    core::context::UiContext,
    message::{ime::ImeMessage, UiMessage, UiMessageType},
};

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize)]
pub struct TextBox {
    text: String,
    focus: bool,
    widget: Widget,
    align: Align2,
    hint_text: String,
}

impl Debug for TextBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextBox").field("text", &self.text).finish()
    }
}

#[typetag::serde]
impl UiNodeTrait for TextBox {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = egui::Frame::none();

        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        let UiContext { ui, sender } = ctx;
        ui.scope(|ui| {
            ui.style_mut().visuals.selection.bg_fill = Color32::from_rgb(51, 103, 209);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(0.5, Color32::BLACK);

            let width = self.width;
            let height = self.height;
            let input = egui::TextEdit::singleline(&mut self.text)
                .text_color(Color32::BLACK)
                .vertical_align(self.align.y())
                .horizontal_align(self.align.x())
                .hint_text(&self.hint_text);

            let result = ui.add_sized([width, height], input);
        });
    }
}

impl TextBox {
    pub fn new(widget: Widget) -> TextBox {
        Self {
            text: String::new(),
            focus: false,
            widget,
            align: Align2::LEFT_TOP,
            hint_text: String::new(),
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_align(mut self, align: Align2) -> Self {
        self.align = align;
        self
    }
    pub fn with_hint_text(mut self, hint_text: &str) -> Self {
        self.hint_text = hint_text.to_string();
        self
    }
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            text: String::new(),
            focus: false,
            widget: Widget::default().with_width(100.).with_height(30.),
            align: Align2::LEFT_TOP,
            hint_text: String::new(),
        }
    }
}
