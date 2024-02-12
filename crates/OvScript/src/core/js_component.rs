use std::{borrow::Cow, fmt::Debug};

use deno_core::{serde_json, v8, JsRealm, Op};
use log::info;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_v8::Serializable;
use OvCore::ecs::component::{Updated, V8};
use OvMacros::Component;

#[derive(Deserialize, Component)]
pub struct JsComponent {
    name: String,
    jsValue: Option<serde_v8::Global>,
}

impl Serialize for JsComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut comp = serializer.serialize_struct(Self::typeName(), 1)?;
        comp.serialize_field("name", &self.name)?;
        comp.serialize_field::<Option<serde_v8::Global>>("jsValue", &None)?;
        comp.end()
    }
}

impl JsComponent {
    pub fn new(name: &str, jsValue: Option<serde_v8::Global>) -> Self {
        Self {
            name: name.to_string(),
            jsValue,
        }
    }

    pub fn getName(&self) -> &str {
        &self.name
    }
    pub fn getValue(&self) -> &Option<serde_v8::Global> {
        &self.jsValue
    }
    pub fn getV8Value(&self) -> v8::Global<v8::Value> {
        let jsValue = self.jsValue.as_ref();
        if let Some(jsValue) = jsValue {
            jsValue.v8_value.clone()
        } else {
            panic!("组件{}丢失JsValue", self.name)
        }
    }
    pub fn setValue(&mut self, jsValue: Option<serde_v8::Global>) {
        self.jsValue = jsValue;
    }
}

impl V8 for JsComponent {
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
