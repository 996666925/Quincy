use std::sync::mpsc::Sender;

use egui::Color32;
use serde::{Deserialize, Serialize};

use OvMacros::Control;
use OvTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use OvWindowing::Window;

use crate::message::UiMessage;

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize, Debug)]
pub struct Label {
    text: String,
    id: Index,
}

#[typetag::serde]
impl UiNodeTrait for Label {
    fn render(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>) {
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
            id:Index::DANGLING
        }
    }

    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
