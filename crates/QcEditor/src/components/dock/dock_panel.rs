use std::cell::Cell;

use egui::{Color32, Frame, Visuals};
use egui_tiles::{Container, Linear, SimplificationOptions, Tabs, Tree};
use serde::{Deserialize, Deserializer, Serialize};
use thunderdome::Arena;
use QcMacros::{external, Control};
use QcTools::message::messageSender::MessageSender;
use QcUI::{
    component::{Button, ToUi, UiNode, UiNodeTrait},
    core::context::UiContext,
    message::UiMessage,
};

use super::{DockItem, DockLayout};

#[derive(Debug)]
pub struct TreeBehavior {
    sender: MessageSender<UiMessage>,
}

#[derive(Control, Serialize, Deserialize, Debug)]
#[external]
pub struct DockPanel {
    pub widget: Widget,
    pub children: Vec<DockLayout>,

    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserializeRoot")]
    pub root: Option<Tree<DockItem>>,
}

fn deserializeRoot<'de, D>(deserializer: D) -> Result<Option<Tree<DockItem>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
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
                    },
                    ui,
                );
            });
        }
    }
}

impl egui_tiles::Behavior<DockItem> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &DockItem) -> egui::WidgetText {
        (&pane.name).into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        SimplificationOptions {
            prune_empty_tabs: true,
            prune_single_child_tabs: true,
            prune_empty_containers: true,
            prune_single_child_containers: true,
            all_panes_must_have_tabs: true,
            join_nested_linear_containers: true,
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut DockItem,
    ) -> egui_tiles::UiResponse {
        let sender = self.sender.clone();

        let mut ctx = UiContext::new(ui, &sender);
        pane.child.render(&mut ctx);

        egui_tiles::UiResponse::None
    }
}

impl Default for DockPanel {
    fn default() -> Self {
        Self {
            widget: Widget::default(),
            children: Vec::new(),
            root: None,
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

    pub fn with_children(mut self, children: Vec<DockLayout>) -> Self {
        self.children = children;

        self
    }

    #[must_use = "必须调用Build方法,否则无法渲染出界面"]
    pub fn build(mut self) -> Self {
        let mut tiles = egui_tiles::Tiles::default();

        let mut container = Linear {
            dir: egui_tiles::LinearDir::Horizontal,
            children: vec![],
            ..Default::default()
        };

        let mut children = vec![];
        children.append(&mut self.children);

        for layout in children {
            let share = layout.share;
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
