use egui::Vec2;
use QcCore::ecs::components::material_render::MaterialRender;
use QcUI::core::context::UiContext;

use super::InspectorTrait;




impl InspectorTrait for MaterialRender {
    fn inspector(&mut self, ctx: &mut UiContext) {
        egui::CollapsingHeader::new("MaterialRender")
        .default_open(true)
        .show(ctx.ui, |ui| {
            ui.group(|ui| {
                ui.set_min_width(ui.available_width() - 5.);
                egui::Grid::new("MaterialRender")
                    .num_columns(2)
                    .spacing(Vec2::new(10., 10.))
                    .show(ui, |ui| {
                        
                    })
            });
        });
    }
}
