use egui::{Color32, Frame, Key, Margin, RichText, Stroke, Vec2, WidgetText};
use enum_variant_eq::{enum_variant_eq_derive::*, *};
use serde::{Deserialize, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::{CursorIcon, Window};

use crate::message::{UiMessage, UiMessageType};

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
    text: String,
    width: f32,
    height: f32,
    background: Color32, // click: Vec<dyn Fn()+'static>,
    fontSize: f32,
    margin: Margin,
    padding: Margin,
    hoverColor: Color32,
    clickColor: Color32,
    isHover: bool,
    isClick: bool,
    id: Index,
}

#[typetag::serde]
impl UiNodeTrait for Button {
    fn renderFrame(&self, ui: &mut egui::Ui) -> egui::Frame {
        let frame = Frame::none()
            .inner_margin(self.padding)
            .outer_margin(self.margin);
        frame
    }

    fn renderInner(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>) {
        let text = RichText::new(&self.text).size(self.fontSize);

        let color = if self.isClick {
            sender.sendMessage(UiMessage(
                self.id,
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

        let result = ui.add_sized([self.width, self.height], button);
        let result = result.interact(egui::Sense::click_and_drag());

        if result.hovered() {
            sender.sendMessage(UiMessage(
                self.id,
                UiMessageType::ButtonMessage(ButtonMessage::Hovered),
            ));
        }

        if result.clicked() {
            sender.sendMessage(UiMessage(
                self.id,
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
            height: 20.,
            width: 40.,
            background: Color32::from_rgb(239, 239, 239),
            fontSize: 12.,
            margin: Default::default(),
            padding: Default::default(),
            hoverColor: Color32::from_rgb(229, 229, 229),
            clickColor: Color32::from_rgb(245, 245, 245),
            isHover: false,
            isClick: false,
            id: Index::DANGLING,
        }
    }
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            // click: Vec::new(),
            ..Default::default()
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }

    // pub fn
}
