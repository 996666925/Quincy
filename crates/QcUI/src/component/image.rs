use std::{fmt::Debug, sync::mpsc::Sender};

use egui::{Color32, Frame, Margin, TextureId};
use egui_extras::RetainedImage;
use serde::{Deserialize, Deserializer, Serialize};

use QcMacros::Control;
use QcTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use QcWindowing::Window;

use crate::message::UiMessage;

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize)]
pub struct Image {
    src: String,
    id: Index,
    width: f32,
    height: f32,
    margin: Margin,

    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserializeTexture")]
    texture: Option<egui_extras::RetainedImage>,
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
    fn renderFrame(&self, ui: &mut egui::Ui) -> egui::Frame {
        let frame = Frame::none().outer_margin(self.margin);

        frame
    }

    fn renderInner(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>)  {
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

impl Image {
    pub fn new() -> Self {
        Self {
            src: String::new(),
            id: Index::DANGLING,
            width: 100.,
            height: 100.,
            texture: None,
            margin: Margin::default(),
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
    pub fn setTexture(&mut self, name: &str, texture: Option<RetainedImage>) {
        self.texture = texture;
        self.src = name.to_string();
    }
}
