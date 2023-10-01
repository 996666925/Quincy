use deno_core::{op_print, serde_v8, v8, Extension, JsRuntime, Op, RuntimeOptions};
use std::ops::{Deref, DerefMut};

use super::{op_addComponent, op_createGameObject, op_getComponent, op_getGameObject};

pub struct JsRuntimeManager {
    js: JsRuntime,
}

impl Deref for JsRuntimeManager {
    type Target = JsRuntime;

    fn deref(&self) -> &Self::Target {
        &self.js
    }
}

impl DerefMut for JsRuntimeManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.js
    }
}

impl JsRuntimeManager {
    pub fn new() -> Self {
        let ext = Extension {
            name: "OvScript",
            ops: std::borrow::Cow::Borrowed(&[
                op_addComponent::DECL,
                op_getComponent::DECL,
                op_getGameObject::DECL,
                op_createGameObject::DECL,
            ]),
            ..Default::default()
        };
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });
        Self { js: runtime }
    }

    pub fn update(&mut self, dt: f32) {
        let context = self.main_context();
        let scope = &mut v8::HandleScope::with_context(self.v8_isolate(), context.clone());
        let context = v8::Local::new(scope, context);

        let global = context.global(scope);
        let updateName = v8::String::new(scope, "__UPDATE__").unwrap();
        let update = global.get(scope, updateName.into()).unwrap();
        let updateFunc = v8::Local::<v8::Function>::try_from(update).unwrap();
        let args = &[serde_v8::to_v8(scope, dt).unwrap().into()];
        let undefined = v8::undefined(scope);
        updateFunc.call(scope, undefined.into(), args);
    }
}

#[cfg(test)]
mod test {
    use deno_core::{v8, Extension, JsRuntime, RuntimeOptions, serde_json};

    #[test]
    pub fn serialize() {
        let ext = Extension {
            name: "OvScript",
            ops: std::borrow::Cow::Borrowed(&[]),
            ..Default::default()
        };
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        }); 

        let result = runtime
            .execute_script_static("ov", "let obj={a:'233333',b:333,show(){return this.b;}};obj")
            .unwrap();
        let scope = &mut runtime.handle_scope();
        // let result = result.open(runtime.v8_isolate());
        let result = v8::Local::<v8::Value>::new(scope, result);
        let result: serde_json::Value = serde_v8::from_v8(scope, result).unwrap();
        
        println!("{:?}", result)
    }

    #[test]
    pub fn deserialize() {}
}
