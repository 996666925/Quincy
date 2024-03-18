use std::cell::Cell;

use egui::{Color32, CursorIcon, Frame, Response, RichText, Visuals};
use egui_tiles::{Container, Linear, LinearDir, SimplificationOptions, Tabs, TileId, Tiles, Tree};
use serde::{Deserialize, Deserializer, Serialize};
use thunderdome::Arena;
use QcMacros::{external, Control};
use QcTools::message::messageSender::MessageSender;
use QcUI::{
    component::{Button, ToUi, UiNode, UiNodeTrait},
    core::context::UiContext,
    message::UiMessage,
};

use super::{DockItem, DockLayout, DockWidget};

#[derive(Debug)]
pub struct TreeBehavior {
    sender: MessageSender<UiMessage>,
    show_tab: bool,
}

#[derive(Control, Debug)]
#[external]
pub struct DockPanel {
    pub widget: Widget,
    pub children: Vec<DockWidget>,
    pub root: Option<Tree<DockItem>>,
    pub orientation: LinearDir,
}

impl Serialize for DockPanel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for DockPanel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[typetag::serde]
impl UiNodeTrait for DockPanel {
    fn renderFrame(&self, ctx: &mut UiContext) -> egui::Frame {
        let frame = Frame::none()
            .fill(self.background)
            .inner_margin(self.padding)
            .outer_margin(self.margin);

        frame
    }

    fn renderInner(&mut self, ctx: &mut UiContext) {
        if let Some(tree) = self.root.as_mut() {
            ctx.ui.scope(|ui| {
                ui.ctx().set_visuals(Visuals::dark());
                tree.ui(
                    &mut TreeBehavior {
                        sender: ctx.sender.clone(),
                        show_tab: true,
                    },
                    ui,
                );
            });
        }
    }
}

impl egui_tiles::Behavior<DockItem> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &DockItem) -> egui::WidgetText {
        //如果tab_title_for_pane被调用说明自带的tab被显示了，需要隐藏我弄的tab
        self.show_tab = false;
        let text = RichText::new(&pane.name).color(Color32::WHITE).size(14.);
        text.into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        SimplificationOptions {
            prune_empty_tabs: true,
            prune_single_child_tabs: true,
            prune_empty_containers: true,
            prune_single_child_containers: true,
            all_panes_must_have_tabs: false,
            join_nested_linear_containers: false,
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut DockItem,
    ) -> egui_tiles::UiResponse {
        let sender = self.sender.clone();

        let show_tab = pane.show_tab && self.show_tab;

        let res = if show_tab {
            Some(pane.tab_ui(ui, &pane.name))
        } else {
            None
        };

        let mut ctx = UiContext::new(ui, &sender);
        pane.child.render(&mut ctx, show_tab);

        //重置状态，避免影响到其他Panel
        self.show_tab = true;

        match res {
            Some(res) => {
                if res.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::Move);
                }
                if res.drag_started() {
                    egui_tiles::UiResponse::DragStarted
                } else {
                    egui_tiles::UiResponse::None
                }
            }
            _ => egui_tiles::UiResponse::None,
        }
    }
}

impl Default for DockPanel {
    fn default() -> Self {
        Self {
            widget: Widget::default(),
            children: Vec::new(),
            root: None,
            orientation: LinearDir::Vertical,
        }
    }
}

impl DockPanel {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            ..Default::default()
        }
    }

    pub fn with_children(mut self, children: Vec<DockWidget>) -> Self {
        self.children = children;

        self
    }

    pub fn with_orientation(mut self, orientation: LinearDir) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use = "必须调用Build方法,否则无法渲染出界面"]
    pub fn build(mut self) -> Self {
        let mut tiles = egui_tiles::Tiles::default();

        let mut container = Linear {
            dir: self.orientation,
            children: vec![],
            ..Default::default()
        };

        let mut children = vec![];
        children.append(&mut self.children);

        for layout in children {
            let share = layout.get_share();
            let id = layout.build(&mut tiles);
            container.children.push(id);
            if share != 0. {
                container.shares.set_share(id, share);
            }
        }

        let container = tiles.insert_container(Container::Linear(container));

        self.root = Some(egui_tiles::Tree::new("my_tree", container, tiles));

        self
    }
}
