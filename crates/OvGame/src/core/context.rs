use OvCore::resources::ResourceManager;
use nalgebra::{Matrix1, Matrix4, Vector3};
use std::sync::{Arc, RwLock};
use OvCore::ecs::components::camera::Camera;
use OvCore::ecs::game_object::GameObject;
use OvCore::ecs::renderer::Renderer;
use OvCore::ecs::{component::Component, components::transform::Transform};

use OvCore::scene_system::scene_manager::SceneManager;
use OvRender::buffers::UniformBuffer;
use OvRender::settings::driver_settings::DriverSettings;
use OvScript::core::JsRuntimeManager;
use OvTools::utils::r#ref::Ref;
use OvUI::core::ui_manager::UiManager;
use OvWindowing::{
    context::device::Device, event_loop::EventLoop, input::input_manager::InputManager,
    settings::device_settings::DeviceSettings, window::OvWindow,
};

pub struct Context {
    pub device: Device,
    pub inputManager: Ref<InputManager>,
    pub uiManager: Ref<UiManager>,
    pub window: Ref<OvWindow>,
    pub renderer: Ref<Renderer>,
    pub sceneManager: Ref<SceneManager>,
    pub engineUBO: Arc<UniformBuffer<[Matrix4<f32>; 3]>>,
    pub jsRuntimeManager: Ref<JsRuntimeManager>,
    pub resourceManager: Arc<ResourceManager>,
}

impl Context {
    pub fn new(window: Ref<OvWindow>, el: &EventLoop<()>) -> Arc<Context> {
        let inputManager = InputManager::new();
        let window_ref = window.clone();
        let window_read = window_ref.try_read().unwrap();
        let device = Device::new(&window_read, DeviceSettings::default());
        let uiManager = UiManager::new(&window_read, el);
        let renderer = Renderer::new(DriverSettings::default());
        let sceneManager = SceneManager::new();
        let engineUBO = Arc::new(UniformBuffer::new(6));
        let jsRuntimeManager = Ref::new(JsRuntimeManager::new());
        let resourceManager = Arc::new(ResourceManager::new());
        Arc::new(Self {
            device,
            inputManager,
            uiManager,
            window,
            renderer,
            sceneManager,
            engineUBO,
            jsRuntimeManager,
            resourceManager,
        })
    }
}
