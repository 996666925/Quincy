use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use deno_core::{
    serde_v8,
    v8::{self, HandleScope},
};
use serde::Serialize;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use OvTools::{eventing::event::EventId, sync::OnceCell, utils::r#ref::Ref};

use crate::window::OvWindow as Window;

use super::event::VirtualMouse;

static INPUTMANAGER: OnceCell<Ref<InputManager>> = OnceCell::new();

#[derive(Debug)]
pub struct InputManager {
    map: HashMap<VirtualKeyCode, ElementState>,
    mousePosition: (f64, f64),
}

impl InputManager {
    pub fn getInstance() -> Ref<InputManager> {
        INPUTMANAGER.get().unwrap().clone()
    }
    pub fn new() -> Ref<InputManager> {
        let input_manager = Ref::new(InputManager {
            map: HashMap::new(),
            mousePosition: (0., 0.),
        });
        INPUTMANAGER.set(input_manager.clone()).unwrap();

        input_manager
    }

    fn onKeyPressed(&mut self, key: VirtualKeyCode) {
        self.map
            .entry(key)
            .and_modify(|value| *value = ElementState::Pressed)
            .or_insert(ElementState::Pressed);
    }

    fn onKeyReleased(&mut self, key: VirtualKeyCode) {
        self.map
            .entry(key)
            .and_modify(|value| *value = ElementState::Released)
            .or_insert(ElementState::Released);
    }

    pub fn isKeyPressed(&self, key: VirtualKeyCode) -> bool {
        if let Some(state) = self.map.get(&key) {
            *state == ElementState::Pressed
        } else {
            false
        }
    }
    pub fn isKeyReleased(&self, key: VirtualKeyCode) -> bool {
        if let Some(state) = self.map.get(&key) {
            *state == ElementState::Released
        } else {
            false
        }
    }
    pub fn lateUpdate(&mut self) {
        self.map.clear();
    }

    pub fn handleEvent(&mut self, event: &WindowEvent, scope: &mut HandleScope) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => {
                match input.state {
                    ElementState::Pressed => {
                        if let Some(key) = input.virtual_keycode {
                            self.onKeyPressed(key);
                        }
                    }
                    ElementState::Released => {
                        if let Some(key) = input.virtual_keycode {
                            self.onKeyReleased(key);
                        }
                    }
                };
                // callback(input);
                Self::postInputMessage(scope, "keyboard", input);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.mousePosition = (position.x, position.y);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                Self::postInputMessage(
                    scope,
                    "mouse",
                    &VirtualMouse {
                        state: *state,
                        button: *button,
                        position: self.mousePosition,
                    },
                );
            }
            _ => {}
        };
    }

    pub fn postInputMessage(scope: &mut v8::HandleScope, name: &str, data: &impl Serialize) {
        let context = scope.get_current_context();

        let global = context.global(scope);
        let funcName = v8::String::new(scope, "__POST_INPUT_MESSAGE__").unwrap();

        let func = global.get(scope, funcName.into()).unwrap();

        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let args = serde_v8::to_v8(scope, data).unwrap();
        let typeName = serde_v8::to_v8(scope, name).unwrap();
        let undefined = v8::undefined(scope).into();
        func.call(scope, undefined, &[typeName, args]);
    }
}

pub struct Input;

impl Input {
    pub fn isKeyPressed(key: VirtualKeyCode) -> bool {
        let input = InputManager::getInstance();
        let input = input.try_read().unwrap();
        input.isKeyPressed(key)
    }

    pub fn isKeyReleased(key: VirtualKeyCode) -> bool {
        let input = InputManager::getInstance();
        let input = input.try_read().unwrap();
        input.isKeyReleased(key)
    }
}
