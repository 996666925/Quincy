use QcTools::message::messageSender::MessageSender;

use crate::{component::Canvas, message::UiMessage};

pub struct UiContext<'a> {
    pub ui: &'a mut egui::Ui,
    pub sender: &'a MessageSender<UiMessage>,
}

impl<'a> UiContext<'a> {
    pub fn new(ui: &'a mut egui::Ui, sender: &'a MessageSender<UiMessage>) -> Self {
        Self { ui, sender }
    }
}



