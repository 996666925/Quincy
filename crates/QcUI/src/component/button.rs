use egui::{Color32, CursorIcon, Frame, Key, Margin, RichText, Stroke, Vec2, WidgetText};
use enum_variant_eq::{enum_variant_eq_derive::*, *};
use serde::{Deserialize, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::Window;

use crate::{
    core::context::UiContext,
    message::{UiMessage, UiMessageType},
};

use super::UiNodeTrait;

#[derive(Debug, Deserialize, Serialize, Clone, EnumVariantEq)]
pub enum ButtonMessage {
    Clicked,
    Hovered,
    Pressed,
    Released,
}

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Button {
    pub widget: Widget,
    pub text: String,
    pub hoverColor: Color32,
    pub clickColor: Color32,
    pub isHover: bool,
    pub isClick: bool,
}

#[typetag::serde]
impl UiNodeTrait for Button {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = Frame::none()
            .inner_margin(self.padding)
            .outer_margin(self.margin);
        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        let text = RichText::new(&self.text).size(self.font_size);
      
        let color = if self.isClick {
            ctx.sender.sendMessage(UiMessage(
                self.uuid,
                UiMessageType::ButtonMessage(ButtonMessage::Pressed),
            ));
            self.clickColor
        } else if self.isHover {
            self.hoverColor
        } else {
            self.background
        };
        let button = egui::Button::new(text)
            .fill(color)
            .stroke(Stroke::new(0.5, Color32::BLACK));

        let result = ctx.ui.add_sized([self.width, self.height], button);
        let result = result.interact(egui::Sense::click_and_drag());

        if result.hovered() {
            ctx.sender.sendMessage(UiMessage(
                self.uuid,
                UiMessageType::ButtonMessage(ButtonMessage::Hovered),
            ));
            ctx.ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        if result.clicked() {
            ctx.sender.sendMessage(UiMessage(
                self.uuid,
                UiMessageType::ButtonMessage(ButtonMessage::Clicked),
            ));
        }

        self.isHover = result.hovered();

        self.isClick = result.is_pointer_button_down_on();
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            text: Default::default(),
            hoverColor: Color32::from_rgb(229, 229, 229),
            clickColor: Color32::from_rgb(245, 245, 245),
            isHover: false,
            isClick: false,
            widget: Widget::default()
                .with_width(100.)
                .with_height(30.)
                .with_background(Color32::from_rgb(239, 239, 239)),
        }
    }
}

impl Button {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            ..Default::default()
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
}
