use std::sync::mpsc::Sender;

use egui::{Color32, Frame, Margin};
use serde::{Deserialize, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::Window;

use crate::message::UiMessage;

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Label {
    text: String,
    id: Index,
    margin: Margin,
    padding: Margin,
}

#[typetag::serde]
impl UiNodeTrait for Label {
    fn renderFrame(&self, ui: &mut egui::Ui) -> egui::Frame {
        let frame = Frame::none()
            .inner_margin(self.padding)
            .outer_margin(self.margin);
        frame
    }
    
    fn renderInner(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>) {
        ui.scope(|ui| {
            ui.visuals_mut().override_text_color = Some(Color32::RED);
            ui.style_mut().wrap = Some(false);
            let label = egui::Label::new(&self.text);

            ui.add(label);
        });
    }
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            id: Index::DANGLING,
            margin: Margin::default(),
            padding: Margin::default(),
        }
    }

    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
