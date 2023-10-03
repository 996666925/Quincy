use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use deno_core::{
    op2,
    serde_json::{self},
    v8, OpState,
};
use log::info;
use thunderdome::Index;
use OvCore::{ecs::component::Component, scene_system::scene_manager::SceneManager};
use OvTools::utils::r#ref::Ref;

use crate::core::{ChangeResult, JsComponent};

#[op2]
#[global]
pub fn op_addComponent<'a>(
    state: Rc<RefCell<OpState>>,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    comp: v8::Local<v8::Value>,
    #[string] compName: &str,
) -> v8::Global<v8::Value> {
    // state.borrow_mut().
    let mut goIndex = Index::DANGLING;
    let mut compIndex = Index::DANGLING;
    let compV8=v8::Global::new(scope, comp);
    //添加组件
    {
        let sceneManager = state.borrow_mut().borrow::<Ref<SceneManager>>().clone();
        let mut sceneManager = sceneManager.try_write().unwrap();

        let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
        if let Some(index) = scene.getGameObject(name) {
            //给组件添加上父对象name
            {
                let key = v8::String::new(scope, "parent").unwrap();
                let value = v8::String::new(scope, name).unwrap();
                let comp = comp.to_object(scope).unwrap();
                comp.set(scope, key.into(), value.into()).unwrap();
            }
            goIndex = index;
            let jsComp = JsComponent::new(compName, compV8.clone().into());
            compIndex = scene[index].addComponent(Component::new(jsComp));
        }
    }
    //调用组件onStart方法
    {
        let comp = comp.to_object(scope).unwrap();

        let onStartName = v8::String::new(scope, "onStart").unwrap();
        let onStart = comp.get(scope, onStartName.into()).unwrap();
        let onStartFunc = v8::Local::<v8::Function>::try_from(onStart).unwrap();
        onStartFunc.call(scope, comp.into(), &[]);
    }
    compV8
}

#[op2]
#[global]
pub fn op_getComponent<'a>(
    state: Rc<RefCell<OpState>>,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    #[string] compName: &str,
) -> v8::Global<v8::Value> {
    let sceneManager = state.borrow_mut().borrow::<Ref<SceneManager>>().clone();
    let mut sceneManager = sceneManager.try_write().unwrap();

    let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
    if let Some(index) = scene.getGameObject(name) {
        if let Some(comp) = scene[index].getComponentByName::<JsComponent>(compName) {
            return comp.getV8Value();
        }
    }

    let null = serde_v8::to_v8(scope, serde_json::Value::Null).unwrap();
    v8::Global::new(scope, null)
}
