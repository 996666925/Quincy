use std::sync::Arc;

use QcTools::utils::r#ref::Ref;
use QcUI::core::ui_manager::UiManager;
use QcWindowing::{context::device::Device, event_loop::EventLoop, settings::DeviceSettings, window::QcWindow};

pub struct Context {
    pub device: Device,
    pub uiManager: Ref<UiManager>,
    pub window: Ref<QcWindow>,
}

impl Context {
    pub fn new(window: Ref<QcWindow>, el: &EventLoop<()>) -> Arc<Context> {
        let window_ref = window.clone();
        let window_read = window_ref.try_read().unwrap();
        let device = Device::new(&window_read, DeviceSettings::default());
        let uiManager = UiManager::new(&window_read, el);

        Arc::new(Self {
            device,
            uiManager,
            window,
        })
    }
}
