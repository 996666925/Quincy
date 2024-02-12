use enum_variant_eq::{enum_variant_eq_derive::*, *};

use serde::{Deserialize, Serialize};
use thunderdome::Index;

use crate::component::ButtonMessage;

use self::ime::ImeMessage;

pub mod ime;

#[derive(Debug, Serialize)]
pub struct UiMessage(pub Index, pub UiMessageType);



#[derive(Debug, Deserialize, Serialize, Clone, EnumVariantEq)]
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


