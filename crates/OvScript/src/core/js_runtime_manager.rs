use deno_core::{Extension, JsRuntime, RuntimeOptions};

pub struct JsRuntimeManager {
    js: JsRuntime,
}

impl JsRuntimeManager {
    pub fn new() -> Self {
        let ext = Extension {
            name: "OvScript",
            ops: std::borrow::Cow::Borrowed(&[]),
            ..Default::default()
        };
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        Self {
            js: runtime
        }
    }
}