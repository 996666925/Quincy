use std::fmt::Debug;

use deno_core::{v8, JsRealm};
use log::info;
use serde::{Deserialize, Serialize};
use OvCore::ecs::component::Updated;
use OvMacros::Component;


#[derive(Serialize, Deserialize, Component)]
pub struct JsComponent {
    name: String,
    jsValue: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct UpdateResult {
    isDirty: bool,
    value: Option<String>,
}

impl JsComponent {
    pub fn new(name: &str, jsValue: &str) -> Self {
        Self {
            name: name.to_string(),
            jsValue: jsValue.to_string(),
        }
    }

    pub fn getValue(&self) -> &str {
        &self.jsValue
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
        let _scope = &mut js.handle_scope(isolate);

        let obj = v8::String::new(_scope, &self.jsValue).unwrap();

        let update = v8::String::new(_scope, "__ONUPDATE__").unwrap();

        let global = _scope.get_current_context().global(_scope);
        let updateFunc = global.get(_scope, update.into()).unwrap();
        let updateFunc = v8::Local::<v8::Function>::try_from(updateFunc).unwrap();
        let dt = serde_v8::to_v8(_scope, dt).unwrap();
        let typeName = serde_v8::to_v8(_scope, &self.name).unwrap();
        let isDirty = updateFunc
            .call(_scope, obj.into(), &[obj.into(), typeName, dt])
            .unwrap();
        let result: UpdateResult = serde_v8::from_v8(_scope, isDirty).unwrap();

        if result.isDirty {
            self.jsValue = result.value.unwrap();
        }
   
    }
}
