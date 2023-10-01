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
pub fn op_addComponent<'a>(
    state: Rc<RefCell<OpState>>,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    comp: v8::Local<v8::Value>,
    #[string] compName: &str,
) {
    let mut goIndex = Index::DANGLING;
    let mut compIndex = Index::DANGLING;
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
            let comp: serde_json::Value = serde_v8::from_v8(scope, comp).unwrap();
            let jsComp = JsComponent::new(compName, comp);
            compIndex = scene[index].addComponent(Component::new(jsComp));
        }
    }
    //调用组件onStart方法
    {
        let context = scope.get_current_context();
        let global = context.global(scope);
        let onStartName = v8::String::new(scope, "__ONSTART__").unwrap();
        let onStart = global.get(scope, onStartName.into()).unwrap();
        let onStartFunc = v8::Local::<v8::Function>::try_from(onStart).unwrap();
        let isDirty = onStartFunc.call(scope, comp.into(), &[]).unwrap();
        let result: ChangeResult = serde_v8::from_v8(scope, isDirty).unwrap();
        if result.isDirty {
            let sceneManager = state.borrow_mut().borrow::<Ref<SceneManager>>().clone();
            let mut sceneManager = sceneManager.try_write().unwrap();

            let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
            if let Some(comp) = scene[goIndex][compIndex].castMut::<JsComponent>() {
                comp.setValue(result.value);
            }
        }
    }
}

#[op2]
pub fn op_getComponent<'a>(
    state: Rc<RefCell<OpState>>,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    #[string] compName: &str,
) -> v8::Local<'a, v8::Value> {
    let sceneManager = state.borrow_mut().borrow::<Ref<SceneManager>>().clone();
    let mut sceneManager = sceneManager.try_write().unwrap();

    let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
    if let Some(index) = scene.getGameObject(name) {
        info!("{:?}", scene[index]);
        if let Some(comp) = scene[index].getComponentByName::<JsComponent>(compName) {
            let obj = serde_v8::to_v8(scope, comp.getValue()).unwrap();
            {
                let obj = obj.to_object(scope).unwrap();
                let global = scope.get_current_context().global(scope);
                let key = v8::String::new(scope, "__COMPONENT__").unwrap();
                let parent = global.get(scope, key.into()).unwrap();
                obj.set_prototype(scope, parent);
            }
            return obj;
        }
    }
    v8::undefined(scope).into()
}
