use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut, Index as IndexOps, IndexMut},
    sync::{Arc, RwLock},
};

use super::{
    component::{Component, ComponentTrait, Named},
    graph::Graph,
};
use deno_core::{v8, JsRealm};
use log::info;
use serde::{Deserialize, Serialize, Serializer};
use thunderdome::{Arena, Generation, Index};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObject {
    pub name: String,
    pub root: Option<Index>,
    pub parent: Option<Index>,
    pub pool: Arena<Component>,
    pub children: Vec<Index>,
    pub active: bool,
    //雪花想要的功能，z轴自定义深度，用于后面的物体显示在最前面
    pub z_buffer: Option<i32>,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            name: "GameObject".to_string(),
            //组件
            //components
            pool: Arena::new(),
            root: None,
            //对象
            //objects
            children: Vec::new(),
            parent: None,
            active: false,
            z_buffer: None,
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
            //id in GameObject Pool
            root: None,
            children: Vec::new(),
            parent: None,
            active: false,
            z_buffer: None,
        }
    }
    pub fn getName(&self) -> &str {
        &self.name
    }

    ///添加对象请使用Scene::add_child_with_parent方法
    ///if you want to add object ,please use Scene::add_child_with_parent method
    pub fn add_child(&mut self, gameobject: Index) {
        self.children.push(gameobject)
    }

    ///移除对象请使用Scene::remove_child_with_parent方法
    ///if you want to remove object ,please use Scene::remove_child_with_parent method
    pub fn remove_child(&mut self, gameobject: Index) {
        self.children.retain(|id| *id != gameobject)
    }

    ///未添加到场景时，组件无法获取到对象
    ///components can't access object when it has not been added in scene
    pub fn addComponent(&mut self, mut component: Component) -> Index {
        component.set_parent(self.root);
        self.pool.insert(component)
    }

    pub fn getComponentId<T: ComponentTrait + Named>(&self) -> Option<Index> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == T::typeName())
            .map(|handle| handle.0)
    }

    pub fn getComponentAndId<T: ComponentTrait + Named>(&self) -> Option<(Index, &T)> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == T::typeName())
            .map(|handle| handle)
            .and_then(|h| Some((h.0, h.1.cast::<T>().unwrap())))
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
            .and_then(|comp| comp.cast_mut::<T>())
    }

    pub fn getComponentByName<T: ComponentTrait>(&self, name: &str) -> Option<&T> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == name)
            .map(|handle| handle.1)
            .and_then(|comp| comp.cast::<T>())
    }
    pub fn getComponentBoxByName(&self, name: &str) -> Option<&dyn ComponentTrait> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == name)
            .map(|handle| handle.1)
            .map(|comp| comp.get_inner())
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
    pub fn set_root(&mut self, index: Index) {
        self.root = Some(index);
        self.active = true;
        for (_, comp) in self.pool.iter_mut() {
            comp.set_parent(self.root);
        }
    }

    pub fn getRoot(&self) -> Option<Index> {
        self.root
    }

    pub fn set_parent(&mut self, index: Option<Index>) {
        self.parent = index;
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use log::info;
    use thunderdome::{Arena, Index};
    use QcMacros::Comp;

    use crate::ecs::{component::Component, components::camera::Camera};

    use super::GameObject;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Comp, Clone, Serialize, Deserialize, Default)]
    struct Example {
        inner: ComponentInner,
    }

    #[test]
    fn addComp() {
        let mut obj = GameObject::default();
        obj.addComponent(Component::Other(Box::new(Example::default())));
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
        obj.addComponent(Component::Other(Box::new(Example::default())));
        let str = ron::to_string(&obj).unwrap();
        println!("{:#?}", str);
        let ex: GameObject = ron::from_str(&str).unwrap();
        println!("{:#?}", ex);
        let example = ex.getComponent::<Example>();

        println!("{:#?}", example);
    }
}
