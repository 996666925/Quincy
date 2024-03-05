use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut, Index as IndexOps, IndexMut},
    sync::{Arc, RwLock},
};

use super::component::{Component, ComponentTrait, Named};
use deno_core::{v8, JsRealm};
use log::info;
use serde::{Deserialize, Serialize, Serializer};
use thunderdome::{Arena, Index};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObject {
    pub name: String,
    pub pool: Arena<Component>,
    pub root: Option<Index>,
    pub children: Arena<GameObject>,
    pub parent: Option<Index>,
    pub active: bool,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            name: "GameObject".to_string(),
            pool: Arena::new(),
            root: None,
            children: Arena::new(),
            parent: None,
            active: true,
        }
    }
}

impl Deref for GameObject {
    type Target = Arena<Component>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for GameObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

impl IndexOps<Index> for GameObject {
    type Output = Component;

    fn index(&self, index: Index) -> &Self::Output {
        &self.pool[index]
    }
}
impl IndexMut<Index> for GameObject {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.pool[index]
    }
}

impl GameObject {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pool: Arena::new(),
            //GameObject Pool中的id
            root: None,
            children: Arena::new(),
            parent: None,
            active: true,
        }
    }
    pub fn getName(&self) -> &str {
        &self.name
    }

    pub fn add_child(&mut self, mut gameobject: GameObject) -> Index {
        gameobject.parent = self.root;
        self.children.insert(gameobject)
    }

    pub fn addComponent(&mut self, mut component: Component) -> Index {
        component.set_parent(self.root);
        self.pool.insert(component)
    }

    pub fn getComponent<T: ComponentTrait + Named>(&self) -> Option<&T> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == T::typeName())
            .map(|handle| handle.1)
            .and_then(|comp| comp.cast::<T>())
    }

    pub fn getComponentMut<T: ComponentTrait + Named>(&mut self) -> Option<&mut T> {
        self.pool
            .iter_mut()
            .find(|handle| handle.1.getName() == T::typeName())
            .map(|handle| handle.1)
            .and_then(|comp| comp.castMut::<T>())
    }

    pub fn getComponentByName<T: ComponentTrait>(&self, name: &str) -> Option<&T> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == name)
            .map(|handle| handle.1)
            .and_then(|comp| comp.cast::<T>())
    }
    pub fn getComponentBoxByName(&self, name: &str) -> Option<&Box<dyn ComponentTrait>> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == name)
            .map(|handle| handle.1)
            .map(|comp| comp.getValue())
    }

    pub fn isActive(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, dt: f32, js: JsRealm, isolate: &mut v8::OwnedIsolate) {
        if self.active {
            for (_, comp) in self.pool.iter_mut() {
                if comp.typetag_name() == "JsComponent" {
                    comp.updateByJs(dt, js.clone(), isolate);
                } else {
                    comp.update(dt);
                }
            }
        }
    }
    pub fn setRoot(&mut self, index: Index) {
        self.root = Some(index);
    }

    pub fn getRoot(&self) -> Option<Index> {
        self.root
    }

    pub fn set_parent(&mut self, index: Index) {
        self.parent = Some(index);
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use log::info;
    use thunderdome::{Arena, Index};
    use QcMacros::Comp;

    use crate::ecs::{
        component::{Component, ComponentInner},
        components::camera::Camera,
    };

    use super::GameObject;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Comp, Clone, Serialize, Deserialize, Default)]
    struct Example {
        inner: ComponentInner,
    }

    #[test]
    fn addComp() {
        let mut obj = GameObject::default();
        obj.addComponent(Component::new(Example::default()));
        let example = obj.getComponent::<Example>();

        println!("{:#?}", example);
    }

    #[typetag::serde(tag = "type")]
    trait Comp: Debug {}

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct Example2 {
        inner: ComponentInner,
        value: i32,
    }
    #[typetag::serde]
    impl Comp for Example2 {}

    #[derive(Debug, Serialize, Deserialize)]
    struct Pack {
        value: Box<dyn Comp>,
    }
    #[test]
    fn serde() {
        let mut obj = GameObject::default();
        obj.addComponent(Component::new(Example::default()));
        let str = ron::to_string(&obj).unwrap();
        println!("{:#?}", str);
        let ex: GameObject = ron::from_str(&str).unwrap();
        println!("{:#?}", ex);
        let example = ex.getComponent::<Example>();

        println!("{:#?}", example);
    }
}
