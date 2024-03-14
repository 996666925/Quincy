use std::sync::mpsc::Sender;

use egui::{Align2, Color32, Margin, Vec2};
use env_logger::fmt::style::Color;
use QcUI::{
    component::{
        Button, ButtonMessage, Canvas, Label, Panel, PanelWindow, TextBox, ToUi, UiNodeTrait,
        Widget,
    },
    core::ui_manager::UiManager,
    prelude::FlexDirection,
};

use crate::{
    components::dock::{DockItem, DockLayout, DockPanel},
    core::message::Page,
};

use super::{message::EditorMessage, project::ProjectConfig};

pub struct EditorPanel {
    canvas: Canvas,
    sender: Sender<EditorMessage>,
}

impl EditorPanel {
    pub fn new(sender: Sender<EditorMessage>) -> Self {
        let mut hub = EditorPanel {
            canvas: Canvas::new(),
            sender,
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        let sender = self.sender.clone();

        let dock = DockPanel::default()
            .with_children(vec![
                DockLayout::default()
                    .with_children(vec![
                        DockItem::new("层级编辑器", {
                            Panel::default()
                                .with_children(vec![Button::default().toUi()])
                                .toUi()
                        })
                        .with_share(0.6),
                        DockItem::new("资源编辑器", {
                            Panel::default()
                                .with_children(vec![Button::default().toUi()])
                                .toUi()
                        })
                        .with_share(0.4),
                    ])
                    .with_share(0.2),
                DockLayout::default()
                    .with_children(vec![
                        DockItem::new("场景编辑器", {
                            Panel::default()
                                .with_children(vec![Button::default().toUi()])
                                .toUi()
                        }).with_share(0.7),
                        DockItem::new("资源预览", {
                            Panel::default()
                                .with_children(vec![Button::default().toUi()])
                                .toUi()
                        }).with_share(0.3),
                    ])
                    .with_share(0.6),
                DockLayout::default()
                    .with_children(vec![DockItem::new("属性检查器", {
                        Panel::default()
                            .with_children(vec![Button::default().toUi()])
                            .toUi()
                    })])
                    .with_share(0.2),
            ])
            .build()
            .toUi();

        Panel::new(
            Widget::default()
                .with_background(Color32::from_rgb(23, 23, 26))
                .with_height(500.)
                .with_width(1000.),
        )
        .with_orientation(FlexDirection::Column)
        .with_children(vec![dock])
        .build(&mut self.canvas);
    }
}

impl PanelWindow for EditorPanel {
    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    fn get_size(&self) -> Vec2 {
        Vec2::INFINITY
    }
}
