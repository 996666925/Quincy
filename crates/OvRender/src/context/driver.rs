use std::{
    ffi::{c_char, c_void, CStr},
    ops::{BitAnd, BitOr},
};

use log::{error, info, warn};

use crate::settings::driver_settings::DriverSettings;

pub struct Driver {}

impl Driver {
    pub fn loadSettings(settings: DriverSettings) {
        unsafe {
            if settings.debugMode {
                let mut flag = 0;
                gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flag);
                if flag & gl::CONTEXT_FLAG_DEBUG_BIT as i32 != 0 {
                    gl::Enable(gl::DEBUG_OUTPUT);
                    gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                    gl::DebugMessageCallback(
                        Some(Self::GlDebugMessageCallback),
                        std::ptr::null_mut(),
                    );
                    gl::DebugMessageControl(
                        gl::DONT_CARE,
                        gl::DONT_CARE,
                        gl::DONT_CARE,
                        0,
                        std::ptr::null(),
                        true as u8,
                    );
                }
            }
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::CullFace(gl::BACK);
        }
    }

    pub extern "system" fn GlDebugMessageCallback(
        source: u32,
        gltype: u32,
        id: u32,
        severity: u32,
        length: i32,
        message: *const c_char,
        userParam: *mut c_void,
    ) {
        if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
            return;
        }
        let message = unsafe { CStr::from_ptr(message) };
        let mut output = String::new();

        output += "OpenGL Debug Message:\n";
        output += &format!("Debug message ({}): {}\n", id, message.to_str().unwrap());

        match source {
            gl::DEBUG_SOURCE_API => output += "Source: API",
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => output += "Source: Window System",
            gl::DEBUG_SOURCE_SHADER_COMPILER => output += "Source: Shader Compiler",
            gl::DEBUG_SOURCE_THIRD_PARTY => output += "Source: Third Party",
            gl::DEBUG_SOURCE_APPLICATION => output += "Source: Application",
            gl::DEBUG_SOURCE_OTHER => output += "Source: Other",
            _ => {}
        }
        output += "\n";
        match gltype {
            gl::DEBUG_TYPE_ERROR => output += "Type: Error",
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => output += "Type: Deprecated Behaviour",
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => output += "Type: Undefined Behaviour",
            gl::DEBUG_TYPE_PORTABILITY => output += "Type: Portability",
            gl::DEBUG_TYPE_PERFORMANCE => output += "Type: Performance",
            gl::DEBUG_TYPE_MARKER => output += "Type: Marker",
            gl::DEBUG_TYPE_PUSH_GROUP => output += "Type: Push Group",
            gl::DEBUG_TYPE_POP_GROUP => output += "Type: Pop Group",
            gl::DEBUG_TYPE_OTHER => output += "Type: Other",
            _ => {}
        }
        output += "\n";
        match severity {
            gl::DEBUG_SEVERITY_HIGH => output += "Severity: High",
            gl::DEBUG_SEVERITY_MEDIUM => output += "Severity: Medium",
            gl::DEBUG_SEVERITY_LOW => output += "Severity: Low",
            gl::DEBUG_SEVERITY_NOTIFICATION => output += "Severity: Notification",
            _ => {}
        }

        match severity {
            gl::DEBUG_SEVERITY_HIGH => error!("{}", output),
            gl::DEBUG_SEVERITY_MEDIUM => warn!("{}", output),
            gl::DEBUG_SEVERITY_LOW => info!("{}", output),
            gl::DEBUG_SEVERITY_NOTIFICATION => info!("{}", output),
            _ => {}
        }
    }
}
