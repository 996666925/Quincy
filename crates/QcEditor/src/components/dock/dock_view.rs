use std::fmt::Debug;

use egui::{Color32, RichText, Rounding, Stroke, Vec2};
use QcUI::core::context::UiContext;

pub trait DockView: Debug {
    fn tab_ui(&self, ui: &mut egui::Ui, text: &str) -> egui::Response {
        ui.scope(|ui| {
            let mut rect = ui.clip_rect();
            rect.max.y = rect.min.y + 24.;
            ui.painter()
                .rect_filled(rect, Rounding::ZERO, Color32::BLACK);
            let width = text.len() / 3 * 14 + 10;
     
            let text = RichText::new(text).size(14.);

            ui.style_mut().spacing.button_padding = Vec2::new(5., 0.);

            ui.add(
                egui::Button::new(text)
                    .min_size(Vec2::new(width as _, 24.))
                    .frame(true)
                    .stroke(Stroke::new(0.5, ui.visuals().widgets.active.bg_fill))
                    .fill(Color32::from_rgb(27, 27, 27))
                    .rounding(Rounding::ZERO)
                    .sense(egui::Sense::drag()),
            )
        })
        .inner
    }

    fn render(&mut self, ctx: &mut UiContext) {}
}
