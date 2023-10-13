use deno_core::{op_print, serde_v8, v8, Extension, JsRuntime, Op, RuntimeOptions};
use std::ops::{Deref, DerefMut};

use super::{
    opSetPosition, opSetRotation, op_addComponent, op_createGameObject, op_getComponent,
    op_getGameObject,
};

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
                opSetPosition::DECL,
                opSetRotation::DECL,
            ]),
            ..Default::default()
        };
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        Self { js: runtime }
    }
}

#[cfg(test)]
mod test {
    use deno_core::{serde_json, v8, Extension, JsRuntime, RuntimeOptions};

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
            .execute_script_static(
                "ov",
                "let obj={a:'233333',b:333,show(){return this.b;}};obj",
            )
            .unwrap();
        let scope = &mut runtime.handle_scope();
        // let result = result.open(runtime.v8_isolate());
        let result = v8::Local::<v8::Value>::new(scope, result);
        let result: serde_json::Value = serde_v8::from_v8(scope, result).unwrap();
        let jsValue = serde_v8::to_v8(scope, result).unwrap();
        let showName = v8::String::new(scope, "show").unwrap();
    }

    #[test]
    pub fn deserialize() {}
}
