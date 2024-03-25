use crate::components::nestable::{Command, HierarchicalDragAndDrop, ItemId};
use std::sync::Arc;
use thunderdome::{Arena, Index};
use QcCore::{ecs::game_object::GameObject, scene_system::scene::Scene};
use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};

#[derive(Debug)]
pub struct LayerPanel {
    context: Arc<Context>,
    list: HierarchicalDragAndDrop,
}

impl DockView for LayerPanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        egui::Frame::none()
            .outer_margin(egui::Margin::same(8.))
            .show(ctx.ui, |ui| {
                self.list.ui(ui, |command| match command {
                    Command::MoveItem {
                        moved_item_id,
                        target_container_id,
                        target_position_index,
                    } => {
                        let mut scene_manager = self.context.scene_manager.try_write().unwrap();
                        let scene = scene_manager.get_current_scene_mut().as_mut().unwrap();

                        let new_parent = if target_container_id.id == Index::DANGLING {
                            None
                        } else {
                            Some(target_container_id.id)
                        };
                        let child_id = moved_item_id.id;

                        let child = &scene[child_id];

                        let parent = child.parent;

                        if parent != new_parent {
                            scene.remove_child_with_parent(child_id, parent);

                            scene.add_child_with_parent_by_id(child_id, new_parent);
                        }
                    }
                    Command::SetSelectedItem(item_id) => {
                        if let Some(item_id) = item_id {
                            let editor_actions = &self.context.editor_actions;

                            editor_actions.target.set(Some(item_id.id))
                        }
                    }
                    _ => {}
                })
            });
    }
}

impl LayerPanel {
    pub fn new(context: Arc<Context>) -> Self {
        let mut list = HierarchicalDragAndDrop::default();

        let mut this = Self {
            context: context.clone(),
            list,
        };

        let mut scene_manager = context.scene_manager.try_write().unwrap();
        let scene = scene_manager.get_current_scene_mut().as_mut().unwrap();

        for obj in scene.children.iter() {
            this.build_layer(scene, obj, this.list.root_id);
        }

        this
    }

    fn build_layer(&mut self, scene: &Scene, obj: &Index, item_id: ItemId) {
        let obj = &scene[*obj];

        let item_id = self
            .list
            .add_container(item_id, obj.getName(), obj.root.unwrap());
        for id in obj.children.iter() {
            self.build_layer(scene, id, item_id);
        }
    }
}

