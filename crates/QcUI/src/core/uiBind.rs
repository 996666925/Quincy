use serde::{Deserialize, Serialize};
use thunderdome::Index;

use crate::message::UiMessageType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UiBind {
    //事件类型
    pub msgType: UiMessageType,
    //游戏对象id
    objId: Index,
    //组件id
    compId: Index,
    //方法名
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
