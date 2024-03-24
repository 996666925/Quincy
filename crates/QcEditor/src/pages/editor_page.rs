use std::sync::{mpsc::Sender, Arc};

use egui::{Align2, Color32, Margin, Vec2};
use QcCore::scene_system::scene::Scene;
use QcTools::utils::r#ref::Ref;
use QcUI::{
    component::{
        Button, ButtonMessage, Canvas, Label, Panel, PanelWindow, TextBox, ToUi, UiNodeTrait,
        Widget,
    },
    core::ui_manager::UiManager,
    prelude::FlexDirection,
};

use crate::{
    components::dock::{DockContainer, DockItem, DockLayout, DockPanel},
    core::{
        context::Context,
        editor_renderer::EditorRenderer,
        message::{EditorMessage, Page},
    },
    panel::{AttrPanel, GamePanel, LayerPanel, NavPanel, ResPanel, ResPreviewPanel, ScenePanel},
};

pub struct EditorPage {
    context: Arc<Context>,
    editor_renderer: Ref<EditorRenderer>,
    canvas: Canvas,
    sender: Sender<EditorMessage>,
}

impl EditorPage {
    pub fn new(
        context: Arc<Context>,
        editor_renderer: Ref<EditorRenderer>,
        sender: Sender<EditorMessage>,
    ) -> Self {
        let mut hub = EditorPage {
            context,
            canvas: Canvas::new(),
            sender,
            editor_renderer,
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        let sender = self.sender.clone();

        let nav_panel = {
            let context = self.context.clone();
            NavPanel::new(context)
        };

        let scene_panel = {
            let context = self.context.clone();
            ScenePanel::new(context, self.editor_renderer.clone())
        };

        let game_panel = {
            let context = self.context.clone();
            GamePanel::new(context, self.editor_renderer.clone())
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
                DockItem::new("导航栏", Box::new(nav_panel))
                    .with_share(0.05)
                    .with_show_tab(false)
                    .into(),
                DockLayout::default()
                    .with_children(vec![
                        DockLayout::default()
                            .with_children(vec![
                                DockItem::new("层级编辑器", Box::new(layer_panel))
                                    .with_share(0.6)
                                    .into(),
                                DockItem::new("资源编辑器", Box::new(res_panel))
                                    .with_share(0.4)
                                    .into(),
                            ])
                            .with_container(DockContainer::linear_vertical())
                            .with_share(0.2)
                            .into(),
                        DockLayout::default()
                            .with_children(vec![
                                DockLayout::default()
                                    .with_children(vec![
                                        DockItem::new("场景编辑器", Box::new(scene_panel)).into(),
                                        DockItem::new("游戏预览", Box::new(game_panel)).into(),
                                    ])
                                    .with_container(DockContainer::Tabs)
                                    .with_share(0.7)
                                    .into(),
                                DockItem::new("资源预览", Box::new(res_preview_panel))
                                    .with_share(0.3)
                                    .into(),
                            ])
                            .with_container(DockContainer::linear_vertical())
                            .with_share(0.6)
                            .into(),
                        DockLayout::default()
                            .with_children(vec![
                                DockItem::new("属性检查器", Box::new(attr_panel)).into()
                            ])
                            .with_container(DockContainer::linear_vertical())
                            .with_share(0.2)
                            .into(),
                    ])
                    .with_share(0.95)
                    .into(),
            ])
            .build()
            .toUi();

        Panel::new(Widget::default().with_background(Color32::from_rgb(23, 23, 26)))
            .with_orientation(FlexDirection::Column)
            .with_children(vec![
                // Label::default()
                //     .with_text("Quincy游戏引擎")
                //     .with_color(Color32::WHITE)
                //     .toUi(),
                dock,
            ])
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
