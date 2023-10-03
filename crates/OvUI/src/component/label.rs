use egui::Color32;
use OvTools::utils::r#ref::Ref;
use OvWindowing::Window;

use super::Component;

pub struct Label {
    text: String,
}

impl Component for Label {
    fn render(&mut self, ui: &mut egui::Ui, window: &Window) {
        ui.scope(|ui| {
            ui.visuals_mut().override_text_color = Some(Color32::RED);
            ui.style_mut().wrap = Some(false);
            let label = egui::Label::new(&self.text);

            ui.add(label);
        });
    }
}

impl Label {
    pub fn new(text: &str) -> Ref<Self> {
        Ref::new(Self {
            text: text.to_string(),
        })
    }

    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
