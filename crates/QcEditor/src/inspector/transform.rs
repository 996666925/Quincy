use QcCore::ecs::{
    component::{BaseComponentTrait, ComponentTrait},
    components::{material_render::MaterialRender, mesh_render::MeshRender, transform::Transform},
};
use QcUI::core::context::UiContext;

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
        ctx.ui.label("Transform");
    }
}

impl InspectorTrait for MeshRender {
    fn inspector(&mut self, ctx: &mut UiContext) {
        ctx.ui.label("MeshRender");
    }
}

impl InspectorTrait for MaterialRender {
    fn inspector(&mut self, ctx: &mut UiContext) {
        ctx.ui.label("MaterialRender");
    }
}
