use std::{cell::RefCell, rc::Rc, string};

use deno_core::{op2, v8, OpState};
use nalgebra::{Vector3, Point3};
use OvCore::{ecs::components::transform::Transform, scene_system::scene::Scene};

#[op2]
#[global]
pub fn opSetPosition<'a>(
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[serde] position: Point3<f32>,
) {
    let transform = this.get_internal_field(scope, 0).unwrap();

    let transform = v8::Local::<v8::External>::try_from(transform).unwrap();
    let transform = transform.value() as *mut Transform;

    let transform = unsafe { &mut *transform };

    transform.setPosition(position);


}




#[op2]
#[global]
pub fn opSetRotation<'a>(
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[serde] rotation: Vector3<f32>,
) {
    let transform = this.get_internal_field(scope, 0).unwrap();

    let transform = v8::Local::<v8::External>::try_from(transform).unwrap();
    let transform = transform.value() as *mut Transform;

    let transform = unsafe { &mut *transform };

    transform.setRotation(rotation);

    
}
