use std::{
    ffi::{c_void, CStr, CString},
    num::NonZeroU32,
};

use glutin::{
    api::egl::device::Device as eglDevice,
    config::Config,
    context::{
        ContextApi, ContextAttributes, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext, Version
    },
    display::{Display, DisplayApiPreference},
    prelude::{GlConfig, GlDisplay},
    surface::{GlSurface, Surface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};

use raw_window_handle::{
    HasRawWindowHandle, HasWindowHandle, RawDisplayHandle, Win32WindowHandle, WindowsDisplayHandle
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::settings::device_settings::DeviceSettings;

pub struct Device {
    vsync: bool,
    pub display: Display,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl Device {
    pub fn new(window: &Window, settings: DeviceSettings) -> Self {
        unsafe {
            let hwnd = window.raw_window_handle();

            let display = Display::new(
                RawDisplayHandle::Windows(WindowsDisplayHandle::empty()),
                DisplayApiPreference::Wgl(Some(hwnd)),
            )
            .expect("创建Display失败");
            let attrs = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(settings.version)))
                .with_debug(settings.debugProfile)
                .build(Some(hwnd));
            let configs = display
                .find_configs(Default::default())
                .expect("创建WglConfig失败");
            let config = configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        && !accum.supports_transparency().unwrap_or(false);
                    if transparency_check || config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .expect("没有合适的GlConfig");

            let context = display.create_context(&config, &attrs).unwrap();

            let attrs = window.build_surface_attributes(Default::default());
            let surface = display.create_window_surface(&config, &attrs).unwrap();
            let context = context.make_current(&surface).expect("创建surfaces失败");
            // surface
            //     .set_swap_interval(&context, SwapInterval::DontWait)
            //     .expect("设置vsync失败");
            gl::load_with(|s| display.get_proc_address(CString::new(s).unwrap().as_c_str()));
            Device {
                vsync: true,
                context,
                display,
                surface,
            }
        }
    }

    pub fn setVsync(&mut self, vsync: bool) {
        self.vsync = vsync;
        let value = if vsync {
            SwapInterval::Wait(NonZeroU32::new(1).unwrap())
        } else {
            SwapInterval::DontWait
        };
        self.surface
            .set_swap_interval(&self.context, value)
            .expect("设置vsync失败");
    }

    pub fn swapBuffers(&self) {
        self.surface
            .swap_buffers(&self.context)
            .expect("交换缓冲区失败");
    }
}
