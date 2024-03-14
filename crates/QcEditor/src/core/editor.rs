use std::{
    cell::Cell,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

use egui::Vec2;
use QcRender::{core::Renderer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::component::PanelWindow;
use QcWindowing::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    settings::WindowSettings,
    window::QcWindow,
    Fullscreen,
};

use crate::managers::page_manager::PageManager;

use super::{
    context::Context, editor_panel::EditorPanel, message::{EditorMessage, Page}, project_hub::TestPanel, project_hub_panel::ProjectHubPanel
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

        let project_hub_panel = Box::new(ProjectHubPanel::new(sender.clone()));

        let editor_panel = Box::new(EditorPanel::new(sender.clone()));

        let mut page_manager = PageManager::new();

        page_manager.add_page(Page::ProjectHub, project_hub_panel);
        page_manager.add_page(Page::Editor, editor_panel);

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
            let mut canvas_list = vec![page.get_canvas()];
            uiManager.render(&window, &mut canvas_list);
            uiManager.update_not_js(canvas_list);
        }

        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                EditorMessage::GoTo(page) => {
                    self.page_manager.navigate_to(page);

                    let page = self.page_manager.get_current().unwrap();
                    let size: Vec2 = page.get_size();
                    if size == Vec2::INFINITY {
                        window.set_maximized(true);
                    } else {
                        window.set_maximized(false);
                        let _ = window.request_inner_size(LogicalSize::new(size.x, size.y));
                    }
                }
            }
        }
    }

    pub fn post_update(&self) {
        self.context.device.swapBuffers();
    }
}
