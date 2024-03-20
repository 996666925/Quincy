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
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use QcTools::{eventing::event::EventId, sync::OnceCell, utils::r#ref::Ref};

use crate::window::QcWindow as Window;

use super::event::{KeyBoardEvent, MouseEvent, MouseMoveEvent};

#[derive(Debug)]
pub struct InputManager {
    key_events: HashMap<KeyCode, ElementState>,
    mouse_events: HashMap<MouseButton, ElementState>,
    mouse_position: (f64, f64),
}

impl InputManager {
    pub fn new() -> Ref<InputManager> {
        let input_manager = Ref::new(InputManager {
            key_events: HashMap::new(),
            mouse_events: HashMap::new(),
            mouse_position: (0., 0.),
        });

        input_manager
    }

    fn onKeyPressed(&mut self, key: KeyCode) {
        self.key_events
            .entry(key)
            .and_modify(|value| *value = ElementState::Pressed)
            .or_insert(ElementState::Pressed);
    }

    fn onKeyReleased(&mut self, key: KeyCode) {
        self.key_events
            .entry(key)
            .and_modify(|value| *value = ElementState::Released)
            .or_insert(ElementState::Released);
    }

    pub fn isKeyPressed(&self, key: KeyCode) -> bool {
        if let Some(state) = self.key_events.get(&key) {
            *state == ElementState::Pressed
        } else {
            false
        }
    }

    pub fn isKeyReleased(&self, key: KeyCode) -> bool {
        if let Some(state) = self.key_events.get(&key) {
            *state == ElementState::Released
        } else {
            false
        }
    }

    pub fn isMousePressed(&self, mouse: MouseButton) -> bool {
        if let Some(state) = self.mouse_events.get(&mouse) {
            *state == ElementState::Pressed
        } else {
            false
        }
    }

    pub fn isMouseReleased(&self, mouse: MouseButton) -> bool {
        if let Some(state) = self.mouse_events.get(&mouse) {
            *state == ElementState::Released
        } else {
            false
        }
    }

    pub fn lateUpdate(&mut self) {
        self.key_events.clear();
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
                match event.state {
                    ElementState::Pressed => {
                        self.onKeyPressed(key);
                    }
                    ElementState::Released => {
                        self.onKeyReleased(key);
                    }
                }

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
                self.mouse_position = (position.x, position.y);
                Self::postInputMessage(
                    scope,
                    "mouse_move",
                    &MouseMoveEvent {
                        state: "Move".into(),
                        position: self.mouse_position,
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
                        position: self.mouse_position,
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
