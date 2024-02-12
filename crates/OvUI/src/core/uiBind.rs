use serde::{Deserialize, Serialize};
use thunderdome::Index;

use crate::message::UiMessageType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UiBind {
    pub msgType: UiMessageType,
    objId: Index,
    compId: Index,
    funcName: String,
}

impl UiBind {
    pub fn new(objId: Index, compId: Index, funcName: String, msgType: UiMessageType) -> Self {
        Self {
            objId,
            compId,
            funcName,
            msgType,
        }
    }
}
