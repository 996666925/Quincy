use std::sync::Arc;

use deno_core::{op2, v8, OpState};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use OvCore::{
    ecs::game_object::GameObject,
    scene_system::{scene::Scene, scene_manager::SceneManager},
};
use OvTools::utils::r#ref::Ref;

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
        let index = go.getRoot();
        let obj = v8::ObjectTemplate::new(scope);
        obj.set_internal_field_count(1);

        let obj = obj.new_instance(scope).unwrap();
        let value = Box::new(index);

        let value = v8::External::new(scope, Box::into_raw(value) as _);
        obj.set_internal_field(0, value.into());
        let key = v8::String::new(scope, "name").unwrap();
        let value = v8::String::new(scope, go.getName()).unwrap();
        obj.set(scope, key.into(), value.into());

        //给GameObject添加上transform属性
        if let Some(transform) = go.getComponentBoxByName("Transform") {
            let transform = transform.toV8Local(scope);
            let transform = transform.to_object(scope).unwrap();
            let global = scope.get_current_context().global(scope);
            let key = v8::String::new(scope, "__Transform__").unwrap();
            let proto = global.get(scope, key.into()).unwrap();

            //transform js对象的原型对象是transform rust对象,给他原型的原型添加上js的扩展方法
            {
                let this = transform.get_prototype(scope).unwrap();
                let this = this.to_object(scope).unwrap();
                this.set_prototype(scope, proto);
            }

            let key = v8::String::new(scope, "transform").unwrap();
            obj.set(scope, key.into(), transform.into());
        }

        return obj.into();
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
    let index = scene.addChild(go);
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
