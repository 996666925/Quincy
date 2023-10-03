use std::{
    ffi::CString,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use OvTools::{eventing::event::Event as OvEvent, utils::r#ref::Ref};

use crate::{context::device::Device, settings::window_settings::WindowSettings};

#[derive(Debug)]
pub struct OvWindow {
    window: Window,
}

impl OvWindow {
    pub fn new(el: &EventLoop<()>, setting: WindowSettings) -> Ref<Self> {
        let window = Self::createWindow(el, setting);

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

        // window.set_ime_position(LogicalPosition::new(0, 0));
        window
    }
}

impl Deref for OvWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
