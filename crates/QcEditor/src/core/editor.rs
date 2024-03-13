use std::{
    cell::Cell,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

use QcRender::{core::Renderer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::component::PanelWindow;
use QcWindowing::{
    event::WindowEvent, event_loop::EventLoop, settings::WindowSettings, window::QcWindow,
};

use crate::managers::page_manager::PageManager;

use super::{
    context::Context,
    message::{EditorMessage, Page},
    project_hub::TestPanel,
    project_hub_panel::ProjectHubPanel,
};

pub struct Editor {
    pub renderer: Arc<Renderer>,
    pub context: Arc<Context>,
    pub page_manager: PageManager,
    pub window: Ref<QcWindow>,
    receiver: Receiver<EditorMessage>,
    sender: Sender<EditorMessage>,
}

impl Editor {
    pub fn new(setting: WindowSettings, el: &EventLoop<()>) -> Self {
        let window = QcWindow::new(&el, setting);

        let context = Context::new(window.clone(), &el);

        let renderer = Arc::new(Renderer::new(DriverSettings::default()));

        let (sender, receiver) = channel();

        let main_panel = Box::new(ProjectHubPanel::new(sender.clone()));

        let mut page_manager = PageManager::new();

        page_manager.add_page(Page::ProjectHub, main_panel);
        page_manager.add_page(Page::Editor, Box::new(TestPanel::new(sender.clone())));

        Self {
            window,
            context,
            renderer,
            page_manager,
            sender,
            receiver,
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

        if let Some(page) = self.page_manager.get_current_mut() {
            page.show(&window, &mut uiManager);
            page.update(&mut uiManager);
        }

        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                EditorMessage::GoTo(page) => {
                    self.page_manager.navigate_to(page);
                }
            }
        }
    }

    pub fn post_update(&self) {
        self.context.device.swapBuffers();
    }
}
