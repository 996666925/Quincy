use std::{any::Any, sync::mpsc::Sender};

use egui::{Frame, Ui};
use serde::{Deserialize, Serialize};
use thunderdome::Index;
use QcCore::ecs::component::BaseComponentTrait;
use QcTools::message::messageSender::MessageSender;

use crate::{core::context::UiContext, message::UiMessage};

#[typetag::serde(tag = "type")]
pub trait UiNodeTrait: BaseComponentTrait + SetId {
    fn render(&mut self, ctx: &mut UiContext) {
        let frame = self.renderFrame(ctx);
        let UiContext { ui, sender } = ctx;

        frame.show(ctx.ui, |ui| {
            self.renderInner(&mut UiContext::new(ui, sender))
        });
    }

    fn renderTop(&mut self, ctx: &mut UiContext) {
        let frame = self.renderFrame(ctx);
        let UiContext { ui, sender } = ctx;

        egui::CentralPanel::default()
            .frame(frame)
            .show(ui.ctx(), move |ui| {
                self.renderInner(&mut UiContext::new(ui, sender))
            });
    }

    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        Frame::none()
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {}
}

pub trait ToUi: Sized + UiNodeTrait {
    fn toUi(self) -> UiNode;
}

impl<T> ToUi for T
where
    Self: UiNodeTrait,
{
    fn toUi(self) -> UiNode {
        UiNode::new(self)
    }
}

pub trait SetId {
    fn setId(&mut self, id: Index);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiNode {
    name: String,
    tag: String,
    class: Vec<String>,
    pub value: Box<dyn UiNodeTrait>,
    //父对象的index
    //index of parent object 
    parent: Option<Index>,
    // name: QualName,
    // data: String,
    // attr: Vec<Attribute>,
    nodes: Vec<Index>,
}

impl UiNode {
    pub fn new(node: impl UiNodeTrait) -> Self {
        UiNode {
            name: "UiNode".to_string(),
            tag: "tag".to_string(),
            class: vec![],
            value: Box::new(node),
            parent: None,
            nodes: Default::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn addChild(&mut self, child: Index) {
        self.nodes.push(child);
    }

    pub fn cast<T: Any>(&self) -> Option<&T> {
        self.value.asAny().downcast_ref::<T>()
    }

    pub fn castMut<T: UiNodeTrait>(&mut self) -> Option<&mut T> {
        self.value.asAnyMut().downcast_mut::<T>()
    }
}
