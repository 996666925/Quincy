use QcCore::ecs::component::{Component, ComponentTrait};
use QcUI::core::context::UiContext;

mod transform;

pub trait InspectorTrait {
    fn inspector(&mut self, ctx: &mut UiContext) {}
}

impl InspectorTrait for Component {
    fn inspector(&mut self, ctx: &mut UiContext) {
        match self {
            Component::Camera(comp) => {}
            Component::Light(comp) => {}
            Component::MaterialRender(comp) => {
                comp.inspector(ctx);
            }
            Component::MeshRender(comp) => {
                comp.inspector(ctx);
            }
            Component::SkyBox(comp) => {}
            Component::Transform(comp) => {
                comp.inspector(ctx);
            }
            Component::Other(comp) => {}
        }
    }
}

pub struct InspectorMap;
