use std::sync::{mpsc::Sender, Arc};

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
    core::{
        context::Context,
        message::{EditorMessage, Page},
    },
    panel::{AttrPanel, GamePanel, LayerPanel, ResPanel, ResPreviewPanel},
};

pub struct EditorPage {
    context: Arc<Context>,
    canvas: Canvas,
    sender: Sender<EditorMessage>,
}

impl EditorPage {
    pub fn new(context: Arc<Context>, sender: Sender<EditorMessage>) -> Self {
        let mut hub = EditorPage {
            context,
            canvas: Canvas::new(),
            sender,
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        let sender = self.sender.clone();

        let game_panel = {
            let context = self.context.clone();
            GamePanel::new(context)
        };

        let layer_panel = {
            let context = self.context.clone();
            LayerPanel::new(context)
        };
        let res_panel = {
            let context = self.context.clone();
            ResPanel::new(context)
        };

        let res_preview_panel = {
            let context = self.context.clone();
            ResPreviewPanel::new(context)
        };

        let attr_panel = {
            let context = self.context.clone();
            AttrPanel::new(context)
        };

        let dock = DockPanel::default()
            .with_children(vec![
                DockLayout::default()
                    .with_children(vec![
                        DockItem::new("层级编辑器", Box::new(layer_panel)).with_share(0.6),
                        DockItem::new("资源编辑器", { Box::new(res_panel) }).with_share(0.4),
                    ])
                    .with_share(0.2),
                DockLayout::default()
                    .with_children(vec![
                        DockItem::new("场景编辑器", { Box::new(game_panel) }).with_share(0.7),
                        DockItem::new("资源预览", { Box::new(res_preview_panel) }).with_share(0.3),
                    ])
                    .with_share(0.6),
                DockLayout::default()
                    .with_children(vec![DockItem::new("属性检查器", {
                        Box::new(attr_panel)
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

impl PanelWindow for EditorPage {
    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    fn get_size(&self) -> Vec2 {
        Vec2::INFINITY
    }
}
