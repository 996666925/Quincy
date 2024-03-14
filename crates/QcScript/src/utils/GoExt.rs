use deno_core::v8;
use QcCore::ecs::game_object::GameObject;

///转换成js对象时，添加必要的属性
///when it converted to js object ,adding necessary property
pub fn toJsValue<'a>(
    scope: &mut v8::HandleScope<'a>,
    go: &GameObject,
) -> v8::Local<'a, v8::Object> {
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
    //adding transform proper to gameobject 
    if let Some(transform) = go.getComponentBoxByName("Transform") {
        let transform = transform.toV8Local(scope);
        let transform = transform.to_object(scope).unwrap();
        
        let global = scope.get_current_context().global(scope);
        let key = v8::String::new(scope, "__Transform__").unwrap();
        let proto = global.get(scope, key.into()).unwrap();

        //transform js对象的原型对象是transform rust对象,给它原型的原型添加上js的扩展方法
        //prototype object of transform js object is transform rust object 
        //,adding js extending method to the prototype of its prototype 
        {
            // let this = transform.get_prototype(scope).unwrap();
            // let this = this.to_object(scope).unwrap();
            transform.set_prototype(scope, proto);
        }

        let key = v8::String::new(scope, "transform").unwrap();
        obj.set(scope, key.into(), transform.into());
    }

    //继承js GameObject
    //inherit js gameobject 
    {
        let obj = obj.to_object(scope).unwrap();
        let global = scope.get_current_context().global(scope);
        let key = v8::String::new(scope, "__GAMEOBJECT__").unwrap();
        let parent = global.get(scope, key.into()).unwrap();
        obj.set_prototype(scope, parent);
    }

    obj
}

pub fn onStart(comp: v8::Local<v8::Value>, scope: &mut v8::HandleScope) {
    {
        let comp = comp.to_object(scope).unwrap();

        let onStartName = v8::String::new(scope, "onStart").unwrap();
        let onStart = comp.get(scope, onStartName.into()).unwrap();
        let onStartFunc = v8::Local::<v8::Function>::try_from(onStart).unwrap();
        onStartFunc.call(scope, comp.into(), &[]);
    }
}

pub fn setParentName(comp: v8::Local<v8::Value>, scope: &mut v8::HandleScope, parent: &str) {
    //给组件添加上父对象name
    //adding parent object name to component 

    let key = v8::String::new(scope, "parent").unwrap();
    let value = v8::String::new(scope, parent).unwrap();
    let comp = comp.to_object(scope).unwrap();
    comp.set(scope, key.into(), value.into()).unwrap();
}
