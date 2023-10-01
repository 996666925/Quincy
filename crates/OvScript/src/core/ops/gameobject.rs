use deno_core::{op2, v8, OpState};
use log::info;
use serde::{Deserialize, Serialize};
use OvCore::{ecs::game_object::GameObject, scene_system::scene_manager::SceneManager};
use OvTools::utils::r#ref::Ref;

#[derive(Serialize, Deserialize)]
struct GO {
    pub name: String,
}
#[op2]
pub fn op_getGameObject<'a>(
    state: &mut OpState,
    _scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[string] name: &str,
) -> v8::Local<'a, v8::Value> {
    let sceneManager = state.borrow::<Ref<SceneManager>>().clone();
    let sceneManager = sceneManager.try_read().unwrap();

    if let Some(scene) = sceneManager.getCurrentScene() {
        if let Some((_, go)) = scene.iter().find(|go| go.1.getName() == name) {
            let index = go.getRoot();
            let obj = v8::ObjectTemplate::new(_scope);
            obj.set_internal_field_count(1);

            let obj = obj.new_instance(_scope).unwrap();
            let value = Box::new(index);

            let value = v8::External::new(_scope, Box::into_raw(value) as _);
            obj.set_internal_field(0, value.into());
            let key = v8::String::new(_scope, "name").unwrap();
            let value = v8::String::new(_scope, go.getName()).unwrap();
            obj.set(_scope, key.into(), value.into());
            return obj.into();
        }
    }
    return v8::undefined(_scope).into();
}

#[op2]
pub fn op_createGameObject<'a>(
    state: &mut OpState,
    _scope: &mut v8::HandleScope<'a>,
    #[string] name: &str,
) -> v8::Local<'a, v8::Value> {
    let sceneManager = state.borrow::<Ref<SceneManager>>().clone();
    let mut sceneManager = sceneManager.try_write().unwrap();

    let mut scene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
    let go = GameObject::new(name);
    let index = scene.addChild(go);
    let go = GO {
        name: name.to_string(),
    };

    let obj = serde_v8::to_v8(_scope, go).unwrap();

    //继承js GameObject
    {
        let obj = obj.to_object(_scope).unwrap();
        let global = _scope.get_current_context().global(_scope);
        let key = v8::String::new(_scope, "__GAMEOBJECT__").unwrap();
        let parent = global.get(_scope, key.into()).unwrap();
        obj.set_prototype(_scope, parent);
    }

    return obj.into();
}
