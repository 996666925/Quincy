use std::{cell::RefCell, ops::Add, rc::Rc, string, vec};

use deno_core::{op2, v8, OpState};
use nalgebra::{Matrix4, Point3, UnitQuaternion, Vector3};
use QcCore::{ecs::components::transform::Transform, scene_system::scene::Scene};

#[op2]
#[serde]
pub fn opGetPosition<'a>(
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
) -> Point3<f32> {
    let transform = this.get_internal_field(scope, 0).unwrap();

    let transform = v8::Local::<v8::External>::try_from(transform).unwrap();
    let transform = transform.value() as *mut Transform;

    let transform = unsafe { &mut *transform };
    transform.position()
}

#[op2]
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
#[serde]
pub fn opGetRotation<'a>(
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
) -> (f32, f32, f32) {
    let transform = this.get_internal_field(scope, 0).unwrap();

    let transform = v8::Local::<v8::External>::try_from(transform).unwrap();
    let transform = transform.value() as *mut Transform;

    let transform = unsafe { &mut *transform };

    transform.rotation().euler_angles()
}

#[op2]
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

#[op2]
pub fn opTranslate<'a>(
    scope: &mut v8::HandleScope<'a>,
    this: v8::Local<v8::Object>,
    #[serde] vector: Vector3<f32>,
) {
    let transform = this.get_internal_field(scope, 0).unwrap();

    let transform = v8::Local::<v8::External>::try_from(transform).unwrap();
    let transform = transform.value() as *mut Transform;

    let transform = unsafe { &mut *transform };
    let rotate = transform.rotation();

    let vector = rotate * vector;

    let position = transform.position() + vector;
    transform.setPosition(position);
}
