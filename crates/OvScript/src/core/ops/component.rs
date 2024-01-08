use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use deno_core::{
    op2,
    serde_json::{self},
    v8, OpState,
};
use log::info;
use serde::Serialize;
use serde_v8::Serializable;
use thunderdome::Index;
use OvCore::{
    ecs::component::Component,
    scene_system::{scene::Scene, scene_manager::SceneManager},
};
use OvTools::utils::r#ref::Ref;

use crate::{core::JsComponent, utils::GoExt};

#[op2]
#[global]
pub fn op_addComponent<'a>(
    state: Rc<RefCell<OpState>>,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    comp: v8::Local<v8::Value>,
    #[string] compName: &str,
) -> v8::Global<v8::Value> {
    let mut goIndex = Index::DANGLING;
    let mut compIndex = Index::DANGLING;
    let compV8 = v8::Global::new(scope, comp);
    //添加组件
    {
        let mut state = state.borrow_mut();
        let scene = state.borrow_mut::<*mut Scene>();
        let scene = unsafe { &mut **scene };
        if let Some(index) = scene.getGameObject(name) {
            goIndex = index;
            let jsComp = JsComponent::new(compName, compV8.clone().into());
            compIndex = scene[index].addComponent(Component::new(jsComp));
        }
    }
    //调用组件onStart方法
    {
        GoExt::setParentName(comp, scope, name);
        GoExt::onStart(comp, scope);
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
    let mut state = state.borrow_mut();
    let scene = state.borrow_mut::<*mut Scene>();
    let scene = unsafe { &mut **scene };

    if let Some(index) = scene.getGameObject(name) {
        if let Some(comp) = scene[index].getComponentBoxByName(compName) {
            return comp.toV8Global(scope);
        }
    }
    let null = serde_v8::to_v8(scope, serde_json::Value::Null).unwrap();
    v8::Global::new(scope, null)
}
