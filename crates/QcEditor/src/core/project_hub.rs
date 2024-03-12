use std::{cell::Cell, sync::Arc};

use QcRender::{core::Renderer, settings::driver_settings::DriverSettings};
use QcTools::utils::r#ref::Ref;
use QcUI::component::PanelWindow;
use QcWindowing::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    settings::WindowSettings,
    window::QcWindow,
};

use super::{context::Context, editor::Editor, project_hub_panel::ProjectHubPanel};

pub struct ProjectHub {
    editor: Editor,
    el: EventLoop<()>,
    width: i32,
    height: i32,
}

impl ProjectHub {
    pub fn new() -> Self {
        Self::setup_context()
        // ProjectHub {}
    }

    fn setup_context() -> Self {
        let el = EventLoop::new().unwrap();

        let setting = WindowSettings::default().with_height(580).with_width(1000);

        let width = setting.width;
        let height = setting.height;

        let editor = Editor::new(setting, &el);

        ProjectHub {
            editor,
            width,
            height,
            el,
        }
    }

    pub fn run(mut self) {
        self.el
            .run(move |event, el| {
                el.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::WindowEvent { window_id, event } => {
                        self.editor.pre_update(&event);

                        match event {
                            WindowEvent::CloseRequested => {
                                el.exit();
                            }
                            WindowEvent::Resized(size) => {
                                // let renderer = self.context.renderer.try_read().unwrap();
                                // renderer.set_viewport(0, 0, size.width as _, size.height as _);
                            }
                            _ => {
                                // println!("event:{:?}", event);
                            }
                        }
                    }
                    Event::AboutToWait => {
                        self.editor.update();
                        self.editor.post_update();
                        // clock.update();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}
