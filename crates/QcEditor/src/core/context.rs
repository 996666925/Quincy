use std::sync::Arc;

use nalgebra::Matrix4;
use QcCore::{ecs::{renderer::Renderer, MvpUbo}, scene_system::scene_manager::SceneManager};
use QcRender::{buffers::UniformBuffer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::core::ui_manager::UiManager;
use QcWindowing::{
    context::device::Device, event_loop::EventLoop, input::input_manager::InputManager,
    settings::DeviceSettings, window::QcWindow,
};

use super::{
    editor_actions::EditorActions, editor_renderer::EditorRenderer, gizmo_behavior::GizmoBehavior,
};

#[derive(Debug)]
pub struct Context {
    pub device: Device,
    pub ui_manager: Ref<UiManager>,
    pub window: Ref<QcWindow>,
    pub renderer: Ref<Renderer>,
    pub scene_manager: Ref<SceneManager>,
    pub engine_ubo: Arc<MvpUbo>,
    pub gizmo_behavior: Ref<GizmoBehavior>,
    pub input_manager: Ref<InputManager>,
    pub editor_actions: Arc<EditorActions>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(window: Ref<QcWindow>, el: &EventLoop<()>) -> Arc<Context> {
        let window_ref = window.clone();
        let window_read = window_ref.try_read().unwrap();
        let device = Device::new(&window_read, DeviceSettings::default());
        let ui_manager = UiManager::new(&window_read, el);
        let renderer = Renderer::new(DriverSettings::default());
        let engine_ubo = Arc::new(UniformBuffer::new(6));
        let scene_manager = SceneManager::new();
        let gizmo_behavior = GizmoBehavior::new();
        let input_manager = InputManager::new();
        let editor_actions = Arc::new(EditorActions::new());

        Arc::new(Self {
            device,
            ui_manager,
            window,
            renderer,
            engine_ubo,
            scene_manager,
            gizmo_behavior,
            input_manager,
            editor_actions,
        })
    }
}
