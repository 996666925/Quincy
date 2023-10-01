use std::{collections::HashMap, ops::Deref};

use deno_core::{op2, v8, OpState};
use log::info;
use OvCore::{ecs::component::Component, scene_system::scene_manager::SceneManager};
use OvTools::utils::r#ref::Ref;

use crate::core::JsComponent;

#[op2]
pub fn op_addComponent<'a>(
    state: &mut OpState,
    _scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    #[string] comp: &str,
    #[string] compName: &str,
) {
    let sceneManager = state.borrow::<Ref<SceneManager>>().clone();
    let mut sceneManager = sceneManager.try_write().unwrap();

    let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
    if let Some(index) = scene.getGameObject(name) {
        let jsComp = JsComponent::new(compName, comp);
        scene[index].addComponent(Component::new(jsComp));
    }
}

#[op2]
pub fn op_getComponent<'a>(
    state: &mut OpState,
    _scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
    #[string] compName: &str,
) -> v8::Local<'a, v8::Value> {
    let sceneManager = state.borrow::<Ref<SceneManager>>().clone();
    let mut sceneManager = sceneManager.try_write().unwrap();

    let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
    if let Some(index) = scene.getGameObject(name) {
        info!("{:?}",scene[index]);
        if let Some(comp) = scene[index].getComponentByName::<JsComponent>(compName) {
            let obj = serde_v8::to_v8(_scope, comp.getValue()).unwrap();
            {
                let obj = obj.to_object(_scope).unwrap();
                let global = _scope.get_current_context().global(_scope);
                let key = v8::String::new(_scope, "__COMPONENT__").unwrap();
                let parent = global.get(_scope, key.into()).unwrap();
                obj.set_prototype(_scope, parent);
            }
            return obj;
        }
    }
    v8::undefined(_scope).into()
}
