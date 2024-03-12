use crate::message::UiMessageType;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use thunderdome::Index;

#[derive(Debug, Deserialize, Serialize)]
pub enum UiBind {
    NativeBind(NativeUiBind),
    JsBind(JsUiBind),
}

impl UiBind {
    pub fn get_msg_type(&self) -> &UiMessageType {
        match self {
            UiBind::NativeBind(bind) => &bind.msgType,
            UiBind::JsBind(bind) => &bind.msgType,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsUiBind {
    //事件类型
    pub msgType: UiMessageType,
    //游戏对象id
    objId: Index,
    //组件id
    compId: Index,
    //方法名
    funcName: String,
}

impl JsUiBind {
    pub fn new(objId: Index, compId: Index, funcName: String, msgType: UiMessageType) -> Self {
        Self {
            objId,
            compId,
            funcName,
            msgType,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NativeUiBind {
    //事件类型
    pub msgType: UiMessageType,
    //方法名
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserializeFunc")]
    func: Box<dyn Fn(UiMessageType)>,
}

fn deserializeFunc<'de, D>(deserializer: D) -> Result<Box<dyn Fn(UiMessageType)>, D::Error>
where
    D: Deserializer<'de>,
{
    unsafe { *Box::from_raw(std::ptr::null_mut()) }
}

impl Debug for NativeUiBind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeUiBind")
            .field("msgType", &self.msgType)
            .finish()
    }
}

impl NativeUiBind {
    pub fn new(msgType: UiMessageType, func: Box<dyn Fn(UiMessageType)>) -> Self {
        Self { msgType, func }
    }

    pub fn call(&self){
        (self.func)(self.msgType);
    }
}
