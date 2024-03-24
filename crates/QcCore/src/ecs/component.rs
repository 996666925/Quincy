use deno_core::{
    serde_v8,
    v8::{self, ObjectTemplate},
    JsRealm, Op,
};
use erased_serde::{serialize_trait_object, Deserializer};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use std::{
    any::{Any, TypeId},
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};
use thunderdome::Index;

use super::components::{
    camera::Camera, light::Light, material_render::MaterialRender, mesh_render::MeshRender,
    skybox::SkyBox, transform::Transform,
};

pub trait V8 {
    fn toV8Global(&self, scope: &mut v8::HandleScope) -> v8::Global<v8::Value>
    where
        Self: Serialize,
        Self: ComponentTrait,
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
        Self: ComponentTrait,
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
pub trait ComponentTrait: BaseComponentTrait + Updated + V8 + ComponentInnerTrait {
    fn getName(&self) -> &str;
}

pub trait ComponentInnerTrait {
    fn set_parent(&mut self, parent: Option<Index>);

    fn get_parent(&mut self) -> Option<Index>;

    fn is_active(&mut self) -> bool;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ComponentInner {
    //父对象的index
    //index of parent object
    pub parent: Option<Index>,
    pub active: bool,
}

impl ComponentInner {
    pub fn set_parent(&mut self, parent: Option<Index>) {
        self.parent = parent;
    }

    pub fn get_parent(&mut self) -> Option<Index> {
        self.parent
    }

    pub fn is_active(&mut self) -> bool {
        self.active
    }
}

// pub struct Component {
//     pub value: Box<dyn ComponentTrait>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum Component {
    Camera(Camera),
    Light(Light),
    MaterialRender(MaterialRender),
    MeshRender(MeshRender),
    SkyBox(SkyBox),
    Transform(Transform),
    Other(Box<dyn ComponentTrait>),
}

impl Component {
    pub fn getName(&self) -> &str {
        let comp = self.get_inner();
        comp.getName()
    }

    pub fn get_inner(&self) -> &dyn ComponentTrait {
        match self {
            Component::Camera(comp) => comp,
            Component::Light(comp) => comp,
            Component::MaterialRender(comp) => comp,
            Component::MeshRender(comp) => comp,
            Component::SkyBox(comp) => comp,
            Component::Transform(comp) => comp,
            Component::Other(comp) => comp.deref(),
        }
    }

    pub fn get_inner_mut(&mut self) -> &mut dyn ComponentTrait {
        match self {
            Component::Camera(comp) => comp,
            Component::Light(comp) => comp,
            Component::MaterialRender(comp) => comp,
            Component::MeshRender(comp) => comp,
            Component::SkyBox(comp) => comp,
            Component::Transform(comp) => comp,
            Component::Other(comp) => comp.deref_mut(),
        }
    }

    pub fn cast<T: 'static>(&self) -> Option<&T> {
        let comp = self.get_inner();
        comp.asAny().downcast_ref::<T>()
    }

    pub fn cast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let comp = self.get_inner_mut();
        comp.asAnyMut().downcast_mut::<T>()
    }
}

impl Deref for Component {
    type Target = dyn ComponentTrait;

    fn deref(&self) -> &Self::Target {
        self.get_inner()
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_inner_mut()
    }
}
