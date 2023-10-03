use super::game_object::GameObject;
use bevy_reflect::Reflect;
use deno_core::{v8, JsRealm};
use erased_serde::{serialize_trait_object, Deserializer};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};
use thunderdome::Index;

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
pub trait ComponentTrait: BaseComponentTrait + Updated {
    fn getName(&self) -> &str;
}
// serialize_trait_object!(ComponentTrait);

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

    pub fn cast<T: ComponentTrait>(&self) -> Option<&T> {
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
