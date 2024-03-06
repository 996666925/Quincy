use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use super::{context::Context, game::Game};
use rust_embed::RustEmbed;
use QcCore::resources::ResourceTrait;
use QcRender::*;
use QcTools::{time::clock::Clock, utils::r#ref::Ref};
use QcUI::{component::Button, mutex::RwLock, Color32, EguiBackend, Frame};
use QcWindowing::{
    dpi::LogicalPosition,
    event::{ElementState, Event, Ime, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    settings::window_settings::WindowSettings,
    window::QcWindow,
    Window, WindowBuilder,
};

pub struct Application {
    window: Ref<QcWindow>,
    context: Arc<Context>,
    game: Game,
    el: EventLoop<()>,
    width: i32,
    height: i32,
}

impl Application {
    pub fn new(path: Option<Box<dyn ResourceTrait + 'static>>) -> Self {
        env_logger::init();
        let el = EventLoop::new().unwrap();

        let setting = WindowSettings::default();

        let width = setting.width;
        let height = setting.height;

        let window = QcWindow::new(&el, setting);

        let context = Context::new(window.clone(), &el);
        if let Some(path) = path {
            context.resourceManager.setPath(path);
        }
        let game = Game::new(context.clone());

        Application {
            el,
            window,
            context,
            game,
            width,
            height,
        }
    }

    pub fn run(mut self) {
        let mut clock = Clock::new();

        self.el
            .run(move |event, el| {
                // control_flow.set_wait_timeout(Duration::ZERO);
                // control_flow.set_wait();
                el.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::WindowEvent { window_id, event } => {
                        self.game.preUpdate(&event);

                        match event {
                            WindowEvent::CloseRequested => {
                                el.exit();
                            }
                            WindowEvent::Resized(size) => {
                                let renderer = self.context.renderer.try_read().unwrap();
                                renderer.set_viewport(0, 0, size.width as _, size.height as _);
                            }
                            _ => {
                                // println!("event:{:?}", event);
                            }
                        }
                    }
                    Event::AboutToWait => {
                        self.game.update(&clock);
                        self.game.postUpdate();
                        clock.update();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }

    pub fn isRunning(&self) -> bool {
        true
    }

    pub fn setPath(&self, value: Box<impl ResourceTrait + 'static>) {
        self.context.resourceManager.setPath(value);
    }
}

pub struct AppBuilder {
    path: Option<Box<dyn ResourceTrait + 'static>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn setPath(mut self, value: impl ResourceTrait + 'static) -> Self {
        self.path = Some(Box::new(value));
        self
    }

    pub fn build(self) -> Application {
        Application::new(self.path)
    }
}
