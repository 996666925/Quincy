use super::game_object::GameObject;
use bevy_reflect::Reflect;
use deno_core::{
    serde_v8,
    v8::{self, ObjectTemplate},
    JsRealm,
};
use erased_serde::{serialize_trait_object, Deserializer};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};
use thunderdome::Index;

pub trait V8 {
    fn toV8Global(&self, scope: &mut v8::HandleScope) -> v8::Global<v8::Value>
    where
        Self: Serialize,
    {
        let objTemp = ObjectTemplate::new(scope);
        objTemp.set_internal_field_count(1);
        let obj = objTemp.new_instance(scope).unwrap();
        obj.set_internal_field(0, v8::External::new(scope, self as *const _ as _).into());
        let this = serde_v8::to_v8(scope, self).unwrap();
        obj.set_prototype(scope, this);
        let obj: v8::Local<v8::Value> = obj.into();
        v8::Global::new(scope, obj)
    }
    fn toV8Local<'a>(&self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value>
    where
        Self: Serialize,
    {
        let objTemp = ObjectTemplate::new(scope);
        objTemp.set_internal_field_count(1);
        let obj = objTemp.new_instance(scope).unwrap();
        obj.set_internal_field(0, v8::External::new(scope, self as *const _ as _).into());

        let this = serde_v8::to_v8(scope, self).unwrap();
        obj.set_prototype(scope, this);
        let obj: v8::Local<v8::Value> = obj.into();
        obj
    }
}

pub trait Updated {
    fn update(&mut self, dt: f32) {}
    fn updateByJs(&mut self, dt: f32, js: JsRealm, isolate: &mut v8::OwnedIsolate) {}
}

pub trait Named {
    fn typeName() -> &'static str;
}

pub trait BaseComponentTrait: Any + Debug {
    fn asAny(&self) -> &dyn Any;
    fn asAnyMut(&mut self) -> &mut dyn Any;
}

#[typetag::serde(tag = "type")]
pub trait ComponentTrait: BaseComponentTrait + Updated + V8 {
    fn getName(&self) -> &str;
}


impl<T> BaseComponentTrait for T
where
    Self: ComponentTrait,
{
    fn asAny(&self) -> &dyn Any {
        self
    }
    fn asAnyMut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    value: Box<dyn ComponentTrait>,
    //父对象的index
    parent: Option<Index>,
}

// impl Serialize for Component {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("Component", 2)?;
//         s.serialize_field("name", &self.value)?;
//         s.serialize_field("parent", &self.parent)?;
//         s.end()
//     }
// }

impl Component {
    pub fn getName(&self) -> &str {
        self.value.getName()
    }

    pub fn new(comp: impl ComponentTrait) -> Self {
        Self {
            value: Box::new(comp),
            parent: None,
        }
    }

    pub fn cast<T: Any>(&self) -> Option<&T> {
        self.value.asAny().downcast_ref::<T>()
    }
    pub fn castMut<T: ComponentTrait>(&mut self) -> Option<&mut T> {
        self.value.asAnyMut().downcast_mut::<T>()
    }

    pub fn getParent(&self) -> Option<Index> {
        self.parent
    }

    pub fn setParent(&mut self, parent: Option<Index>) {
        self.parent = parent;
    }

    pub fn getValue(&self) -> &Box<dyn ComponentTrait> {
        &self.value
    }
}

impl Deref for Component {
    type Target = dyn ComponentTrait;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.deref_mut()
    }
}
