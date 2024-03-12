use std::{cell::Cell, sync::Arc};

use QcRender::{core::Renderer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::component::PanelWindow;
use QcWindowing::{
    event::WindowEvent, event_loop::EventLoop, settings::WindowSettings, window::QcWindow,
};

use super::{context::Context, project_hub_panel::ProjectHubPanel};

pub struct Editor {
    pub renderer: Arc<Renderer>,
    pub context: Arc<Context>,
    pub window: Ref<QcWindow>,
    pub main_panel: Box<dyn PanelWindow>,
}

impl Editor {
    pub fn new(setting: WindowSettings, el: &EventLoop<()>) -> Self {
        let window = QcWindow::new(&el, setting);

        let context = Context::new(window.clone(), &el);

        let renderer = Arc::new(Renderer::new(DriverSettings::default()));

        let main_panel = Box::new(ProjectHubPanel::new());

        Self {
            window,
            context,
            renderer,
            main_panel,
        }
    }

    pub fn pre_update(&self, event: &WindowEvent) {
        let window = self.context.window.try_read().unwrap();

        let result = self
            .context
            .uiManager
            .try_write()
            .unwrap()
            .handleEvent(&window, event);

        match event {
            WindowEvent::MouseInput { state, .. } => {}
            _ => {
                if result.consumed {
                    return;
                }
            }
        }
    }
    pub fn update(&mut self) {
        self.renderer.setClearColor(0.66, 0.66, 0.66, 1.);
        self.renderer.clear(true, true, false);

        let mut uiManager = self.context.uiManager.try_write().unwrap();
        let window = self.window.try_read().unwrap();

        self.main_panel.show(&window, &mut uiManager);
        self.main_panel.update(&mut uiManager);
    }

    pub fn post_update(&self) {
        self.context.device.swapBuffers();
    }
}
