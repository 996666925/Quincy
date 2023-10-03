use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use OvTools::{eventing::event::EventId, sync::OnceCell, utils::r#ref::Ref};

use crate::window::OvWindow as Window;

static INPUTMANAGER: OnceCell<Ref<InputManager>> = OnceCell::new();

#[derive(Debug)]
pub struct InputManager {
    map: HashMap<VirtualKeyCode, ElementState>,
    keyPressListener: Option<EventId>,
}

impl InputManager {
    pub fn getInstance() -> Ref<InputManager> {
        INPUTMANAGER.get().unwrap().clone()
    }
    pub fn new() -> Ref<InputManager> {
        let input_manager = Ref::new(InputManager {
            map: HashMap::new(),
            keyPressListener: None,
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

    pub fn handleEvent(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,  } => match input.state {
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
            },
            _ => {}
        }
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
