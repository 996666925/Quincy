use std::collections::HashMap;

use enum_variant_eq::{enum_variant_eq_derive::*, *};

use serde::{Deserialize, Serialize};
use thunderdome::Index;
use uuid::Uuid;

use crate::{
    component::{ButtonMessage, Canvas, Widget},
    core::uiBind::{NativeUiBind, UiBind},
};

use self::ime::ImeMessage;

pub mod ime;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct UiMessage(pub Uuid, pub UiMessageType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, EnumVariantEq)]
pub enum UiMessageType {
    Default,
    ButtonMessage(ButtonMessage),
    ImeMessage(ImeMessage),
}

impl PartialEq for UiMessageType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ButtonMessage(l0), Self::ButtonMessage(r0)) => l0.enum_variant_eq(r0),
            _ => self.enum_variant_eq(other),
        }
    }
}

pub struct EventBuilder {
    pub ui_bind_list: HashMap<Uuid, Vec<UiBind>>,
    pub widget: Widget,
}

impl EventBuilder {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            ui_bind_list: HashMap::new(),
        }
    }

    pub fn on_event(mut self, event: UiMessageType, func: Box<dyn Fn(UiMessageType)>) -> Self {
        self.add_ui_bind(self.widget.uuid, NativeUiBind::new(event, func));

        self
    }

    fn add_ui_bind(&mut self, comp: Uuid, bind: NativeUiBind) {
        let bind = UiBind::NativeBind(bind);
        self.ui_bind_list.entry(comp).or_insert(vec![]).push(bind);
    }

    pub fn build(self, canvas: &mut Canvas) -> Widget {
        for bind in self.ui_bind_list {
            canvas.add_ui_bind_list(self.widget.uuid, bind.1);
        }
        self.widget
    }
}
