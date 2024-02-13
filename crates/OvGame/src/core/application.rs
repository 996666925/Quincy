use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use super::{context::Context, game::Game};
use rust_embed::RustEmbed;
use OvCore::resources::ResourceTrait;
use OvRender::*;
use OvTools::{time::clock::Clock, utils::r#ref::Ref};
use OvUI::{component::Button, mutex::RwLock, Color32, EguiBackend, Frame};
use OvWindowing::{
    dpi::LogicalPosition,
    event::{ElementState, Event, Ime, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    settings::window_settings::WindowSettings,
    window::OvWindow,
    Window, WindowBuilder,
};



pub struct Application {
    window: Ref<OvWindow>,
    context: Arc<Context>,
    game: Game,
    el: EventLoop<()>,
    width: i32,
    height: i32,
}

impl Application {
    pub fn new(path: Option<Box<dyn ResourceTrait + 'static>>) -> Self {
        env_logger::init();
        let el = EventLoop::new();

        let setting = WindowSettings::default();

        let width = setting.width;
        let height = setting.height;

        let window = OvWindow::new(&el, setting);
        
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

        self.el.run(move |event, el, control_flow| {
            // control_flow.set_wait_timeout(Duration::ZERO);
            // control_flow.set_wait();

            control_flow.set_poll();
            match event {
                Event::WindowEvent { window_id, event } => {
                    self.game.preUpdate(&event);

                    match event {
                        WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        _ => {
                            // println!("event:{:?}", event);
                        }
                    }
                }
                Event::MainEventsCleared => {}
                Event::RedrawRequested(_) => {}
                Event::RedrawEventsCleared => {
                    self.game.update(&clock);
                    self.game.postUpdate();
                    clock.update();
                }

                _ => {}
            }
        });
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
