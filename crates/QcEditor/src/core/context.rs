use std::sync::Arc;

use QcCore::ecs::renderer::Renderer;
use QcRender::{buffers::UniformBuffer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::core::ui_manager::UiManager;
use QcWindowing::{
    context::device::Device, event_loop::EventLoop, settings::DeviceSettings, window::QcWindow,
};
use nalgebra::Matrix4;
use super::editor_render::EditorRender;

#[derive(Debug)]
pub struct Context {
    pub device: Device,
    pub uiManager: Ref<UiManager>,
    pub window: Ref<QcWindow>,
    pub renderer: Ref<Renderer>,
    pub engineUBO: Arc<UniformBuffer<[Matrix4<f32>; 3]>>,
}

unsafe impl Send for Context{}
unsafe impl Sync for Context{}

impl Context {
    pub fn new(window: Ref<QcWindow>, el: &EventLoop<()>) -> Arc<Context> {
        let window_ref = window.clone();
        let window_read = window_ref.try_read().unwrap();
        let device = Device::new(&window_read, DeviceSettings::default());
        let uiManager = UiManager::new(&window_read, el);
        let renderer = Renderer::new(DriverSettings::default());
        let engineUBO = Arc::new(UniformBuffer::new(6));

        Arc::new(Self {
            device,
            uiManager,
            window,
            renderer,
            engineUBO
        })
    }
}
