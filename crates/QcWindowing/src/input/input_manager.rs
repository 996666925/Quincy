use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use deno_core::{
    serde_v8,
    v8::{self, HandleScope},
};
use serde::Serialize;
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use QcTools::{eventing::event::EventId, sync::OnceCell, utils::r#ref::Ref};

use crate::window::QcWindow as Window;

use super::event::{KeyBoardEvent, MouseEvent, MouseMoveEvent};

static INPUTMANAGER: OnceCell<Ref<InputManager>> = OnceCell::new();

#[derive(Debug)]
pub struct InputManager {
    map: HashMap<KeyCode, ElementState>,
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

    fn onKeyPressed(&mut self, key: KeyCode) {
        self.map
            .entry(key)
            .and_modify(|value| *value = ElementState::Pressed)
            .or_insert(ElementState::Pressed);
    }

    fn onKeyReleased(&mut self, key: KeyCode) {
        self.map
            .entry(key)
            .and_modify(|value| *value = ElementState::Released)
            .or_insert(ElementState::Released);
    }

    pub fn isKeyPressed(&self, key: KeyCode) -> bool {
        if let Some(state) = self.map.get(&key) {
            *state == ElementState::Pressed
        } else {
            false
        }
    }
    
    pub fn isKeyReleased(&self, key: KeyCode) -> bool {
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
                is_synthetic,
                event,
            } => {
                let key = match event.physical_key {
                    PhysicalKey::Code(key) => key,
                    _ => {
                        todo!()
                    }
                };
                // match event.state {
                //     ElementState::Pressed => {
                //         if let PhysicalKey::Code(key) = event.physical_key {
                //             self.onKeyPressed(key);
                //         }
                //     }
                //     ElementState::Released => {

                //             self.onKeyReleased(key);
                //         }
                //     }
                // }

                // callback(input);
                Self::postInputMessage(
                    scope,
                    "keyboard",
                    &KeyBoardEvent {
                        key,
                        state: event.state,
                    },
                );
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.mousePosition = (position.x, position.y);
                Self::postInputMessage(
                    scope,
                    "mouse_move",
                    &MouseMoveEvent {
                        state: "Move".into(),
                        position: self.mousePosition,
                    },
                );
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
                    &MouseEvent {
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
        let funcName = v8::String::new(scope, "__POST_MESSAGE__").unwrap();

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
    pub fn isKeyPressed(key: KeyCode) -> bool {
        let input = InputManager::getInstance();
        let input = input.try_read().unwrap();
        input.isKeyPressed(key)
    }

    pub fn isKeyReleased(key: KeyCode) -> bool {
        let input = InputManager::getInstance();
        let input = input.try_read().unwrap();
        input.isKeyReleased(key)
    }
}
