use QcWindowing::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    settings::WindowSettings,
};

use super::editor::Editor;

pub struct Application {
    editor: Editor,
    el: EventLoop<()>,
}

impl Application {
    pub fn new() -> Self {
        let mut el = EventLoop::new().unwrap();

        let setting = WindowSettings::default().with_height(580).with_width(1000);

        let editor = Editor::new(setting, &el);

        Application { editor, el }
    }

    pub fn run(mut self) {
        self.el
            .run_on_demand(move |event, el| {
                el.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::WindowEvent { window_id, event } => {
                        self.editor.pre_update(&event);

                        match event {
                            WindowEvent::CloseRequested => {
                                el.exit();
                            }
                            WindowEvent::Resized(size) => {}
                            WindowEvent::RedrawRequested => {
                                self.editor.update();
                                self.editor.post_update();
                            }
                            _ => {}
                        }
                    }

                    Event::AboutToWait => {

                        // clock.update();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}
