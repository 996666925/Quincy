use std::sync::Arc;

use deno_core::{op2, v8, OpState};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use thunderdome::Index;
use QcCore::{
    ecs::game_object::GameObject,
    scene_system::{scene::Scene, scene_manager::SceneManager},
};
use QcTools::utils::r#ref::Ref;

use crate::utils::GoExt;

#[derive(Serialize, Deserialize)]
struct GO {
    pub name: String,
}
#[op2]
pub fn op_getGameObject<'a>(
    state: &mut OpState,
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[string] name: &str,
) -> v8::Local<'a, v8::Value> {
    let scene = state.borrow_mut::<*mut Scene>();
    let scene = unsafe { &mut **scene };
    if let Some((_, go)) = scene.iter().find(|go| go.1.getName() == name) {
        return GoExt::toJsValue(scope, go).into();
    }

    return v8::undefined(scope).into();
}

#[op2]
pub fn op_getGameObjectById<'a>(
    state: &mut OpState,
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[serde] id: Index,
) -> v8::Local<'a, v8::Value> {
    let scene = state.borrow_mut::<*mut Scene>();
    let scene = unsafe { &mut **scene };
    if let Some((_, go)) = scene.iter().find(|go| go.1.getRoot().unwrap() == id) {
        return GoExt::toJsValue(scope, go).into();
    }

    return v8::undefined(scope).into();
}

#[op2]
pub fn op_createGameObject<'a>(
    state: &mut OpState,
    scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
) -> v8::Local<'a, v8::Value> {
    let scene = state.borrow_mut::<*mut Scene>();
    let scene = unsafe { &mut **scene };
    let go = GameObject::new(name);
    let index = scene.add_child(go);
    let go = GO {
        name: name.to_string(),
    };

    let obj = serde_v8::to_v8(scope, go).unwrap();

    //继承js GameObject
    {
        let obj = obj.to_object(scope).unwrap();
        let global = scope.get_current_context().global(scope);
        let key = v8::String::new(scope, "__GAMEOBJECT__").unwrap();
        let parent = global.get(scope, key.into()).unwrap();
        obj.set_prototype(scope, parent);
    }

    obj.into()
}
