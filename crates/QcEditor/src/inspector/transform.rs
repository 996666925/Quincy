use egui::Vec2;
use nalgebra::{Point, Point3, Vector3};
use QcCore::ecs::{
    component::{BaseComponentTrait, ComponentTrait},
    components::{material_render::MaterialRender, mesh_render::MeshRender, transform::Transform},
};
use QcUI::{component::Panel, core::context::UiContext};

use super::InspectorTrait;

trait AsInspectorTrait {
    fn as_inspector(&self) -> &dyn InspectorTrait; //返回 Base trait对象
}

// blanket implementation
// 为所有实现 Base 的 T 来实现 AsBase
impl<T: InspectorTrait> AsInspectorTrait for T {
    // 返回 Base trait对象
    fn as_inspector(&self) -> &dyn InspectorTrait {
        self
    }
}

impl InspectorTrait for Transform {
    fn inspector(&mut self, ctx: &mut UiContext) {
        let mut position_x = self.position.x;
        let mut position_y = self.position.y;
        let mut position_z = self.position.z;

        let mut rotation_x = self.rotation.x;
        let mut rotation_y = self.rotation.y;
        let mut rotation_z = self.rotation.z;

        let mut scale_x = self.scale.x;
        let mut scale_y = self.scale.y;
        let mut scale_z = self.scale.z;

        egui::CollapsingHeader::new("Transform")
            .default_open(true)
            .show(ctx.ui, |ui| {
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width() - 5.);
                    egui::Grid::new("TextLayoutDemo")
                        .num_columns(2)
                        .spacing(Vec2::new(10., 10.))
                        .show(ui, |ui| {
                            ui.label("Position:");

                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut position_x).speed(0.01));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut position_y).speed(0.01));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut position_z).speed(0.01));
                            });
                            ui.end_row();

                            ui.label("Rotation:");

                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut rotation_x));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut rotation_y));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut rotation_z));
                            });
                            ui.end_row();

                            ui.label("Scale:");

                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut scale_x).speed(0.01));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut scale_y).speed(0.01));
                                ui.add_space(10.);
                                ui.add(egui::DragValue::new(&mut scale_z).speed(0.01));
                            });
                            ui.end_row();
                        })
                });
            });

        if position_x != self.position.x
            || position_y != self.position.y
            || position_z != self.position.z
        {
            self.set_position(Point3::new(position_x, position_y, position_z));
        }

        if rotation_x != self.rotation.x
            || rotation_y != self.rotation.y
            || rotation_z != self.rotation.z
        {
            self.set_rotation(Vector3::new(rotation_x, rotation_y, rotation_z));
        }

        if scale_x != self.scale.x || scale_y != self.scale.y || scale_z != self.scale.z {
            self.set_scale(Vector3::new(scale_x, scale_y, scale_z));
        }
    }
}

