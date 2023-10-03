use OvTools::utils::r#ref::Ref;
use OvWindowing::{dpi::LogicalPosition, Window};

use super::Component;

pub struct Input {
    text: String,
}

impl Component for Input {
    fn render(&mut self, ui: &mut egui::Ui, window: &Window) {
        let input = egui::TextEdit::singleline(&mut self.text);
        let result = ui.add(input);

        if result.gained_focus() {
            println!("获取焦点");
            window.set_ime_allowed(true);
            window.request_redraw();
            // window.set_ime_allowed(true);
            // window.set_ime_position(LogicalPosition::new(100., 100.));

            // Window::set_ime_allowed(&self, allowed)
        }

        if result.lost_focus() {
            println!("失去焦点");
            // window.set_ime_position(LogicalPosition::new(10000., 10000.));
            window.set_ime_allowed(false);
            // Window::set_ime_allowed(&self, allowed)
        }
    }
}

impl Input {
    pub fn new(text: &str) -> Ref<Input> {
        Ref::new(Self {
            text: text.to_string(),
        })
    }
}
