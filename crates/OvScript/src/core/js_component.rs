use std::fmt::Debug;

use deno_core::{serde_json, v8, JsRealm};
use log::info;
use serde::{Deserialize, Serialize};
use OvCore::ecs::component::{Updated, V8};
use OvMacros::Component;
use serde_v8::Serializable;




#[derive(Serialize, Deserialize, Component)]
pub struct JsComponent {
    name: String,
    jsValue: serde_v8::Global,
}


impl JsComponent {
    pub fn new(name: &str, jsValue: serde_v8::Global) -> Self {
        Self {
            name: name.to_string(),
            jsValue,
        }
    }

    pub fn getValue(&self) -> &serde_v8::Global {
        &self.jsValue
    }
    pub fn getV8Value(&self) -> v8::Global<v8::Value> {
        self.jsValue.v8_value.clone()
    }
    pub fn setValue(&mut self, jsValue: serde_v8::Global) {
        self.jsValue = jsValue;
    }
}

impl V8 for JsComponent{
    fn toV8Global(&self, scope: &mut v8::HandleScope) -> v8::Global<v8::Value> {
        self.getV8Value()
    }
}

impl Debug for JsComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsComponent")
            .field("name", &self.name)
            .finish()
    }
}

impl Updated for JsComponent {
    fn updateByJs(&mut self, dt: f32, js: JsRealm, isolate: &mut v8::OwnedIsolate) {
        let scope = &mut js.handle_scope(isolate);

        let obj = v8::Local::<v8::Value>::new(scope, self.getV8Value());
        let obj = obj.to_object(scope).unwrap();
        

        let update = v8::String::new(scope, "onUpdate").unwrap();
        let onUpdate = obj.get(scope, update.into()).unwrap();

        let onUpdateFunc = v8::Local::<v8::Function>::try_from(onUpdate).unwrap();
        let dt = serde_v8::to_v8(scope, dt).unwrap();
        onUpdateFunc.call(scope, obj.into(), &[dt]);
    }
}
