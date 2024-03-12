use std::ops::Deref;

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use QcTools::utils::r#ref::Ref;

use crate::settings::WindowSettings;

#[derive(Debug)]
pub struct QcWindow {
    window: Window,
}

impl QcWindow {
    pub fn new(el: &EventLoop<()>, setting: WindowSettings) -> Ref<Self> {
        let window = Self::createWindow(el, setting);
        // window.set_ime_allowed(true);
        Ref::new(Self { window })
    }
    pub fn handle(&self) -> &Window {
        &self.window
    }

    fn createWindow(el: &EventLoop<()>, setting: WindowSettings) -> Window {
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(setting.width, setting.height))
            .with_fullscreen(setting.fullscreen)
            .with_resizable(setting.resizable)
            .with_visible(setting.visible)
            .with_title(setting.title)
            .build(el)
            .expect("创建窗口失败");
        // window.set_ime_allowed(true);
        // window.set_cursor_visible(false);
        // window.set_ime_position(LogicalPosition::new(0, 0));

        window
    }
}

impl Deref for QcWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
