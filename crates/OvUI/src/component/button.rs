use OvTools::utils::r#ref::Ref;
use OvWindowing::Window;

use super::Component;

pub struct Button {
    text: String,
    // click: Vec<dyn Fn()+'static>,
}

impl Component for Button {
    fn render(&mut self, ui: &mut egui::Ui, window: &Window) {
        let button = egui::Button::new(&self.text);
        let result = ui.add(button);
        if result.clicked() {
            window.set_ime_allowed(true);
        }
    }
}

impl Button {
    pub fn new(text: &str) -> Ref<Self> {
        Ref::new(Self {
            text: text.to_string(),
            // click: Vec::new(),
        })
    }

    pub fn setText(&mut self, text: &str) {
        self.text = text.to_string();
    }

    // pub fn
}
