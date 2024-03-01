use std::{fmt::Debug, sync::mpsc::Sender};

use egui::{Color32, Frame, Margin, TextureId};
use egui_extras::RetainedImage;
use serde::{Deserialize, Deserializer, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::Window;

use crate::{core::context::UiContext, message::UiMessage};

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize)]
pub struct Image {
    pub src: String,
    pub widget: Widget,

    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserializeTexture")]
    pub texture: Option<egui_extras::RetainedImage>,
}

fn deserializeTexture<'de, D>(
    deserializer: D,
) -> Result<Option<egui_extras::RetainedImage>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("src", &self.src)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[typetag::serde]
impl UiNodeTrait for Image {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = Frame::none().outer_margin(self.margin);

        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        let UiContext { ui, sender } = ctx;
        if let Some(texture) = &self.texture {
            ui.set_width(self.width);
            ui.set_height(self.height);
            let image = egui::Image::new(
                texture.texture_id(ui.ctx()),
                egui::Vec2::new(self.width, self.height),
            );

            ui.add(image);
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            src: String::new(),
            widget: Default::default(),
            texture: None,
        }
    }
}

impl Image {
    pub fn new(widget: Widget) -> Self {
        Self {
            src: String::new(),
            widget,
            texture: None,
        }
    }

    pub fn with_texture(mut self, name: &str, texture: Option<RetainedImage>) -> Self {
        self.texture = texture;
        self.src = name.to_string();
        self
    }

    pub fn setTexture(&mut self, name: &str, texture: Option<RetainedImage>) {
        self.texture = texture;
        self.src = name.to_string();
    }
}
