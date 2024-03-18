use egui_tiles::{TileId, Tiles};

use super::{DockItem, DockLayout};

#[derive(Debug)]
pub enum DockWidget {
    Layout(DockLayout),
    Item(DockItem),
}

impl DockWidget {

    pub fn get_name(&self) -> &str {
        match self {
            DockWidget::Layout(layout) => "DockLayout",
            DockWidget::Item(item) => &item.name,
        }
    }

    pub fn get_share(&self) -> f32 {
        match self {
            DockWidget::Layout(layout) => layout.share,
            DockWidget::Item(item) => item.share,
        }
    }

    pub fn set_show_tab(&mut self, show: bool) {
        match self {
            DockWidget::Layout(_) => {}
            DockWidget::Item(item) => item.show_tab = show,
        }
    }

    pub fn build(self, tiles: &mut Tiles<DockItem>) -> TileId {
        match self {
            DockWidget::Layout(layout) => layout.build(tiles),
            DockWidget::Item(item) => tiles.insert_pane(item),
        }
    }
}

impl Into<DockWidget> for DockLayout {
    fn into(self) -> DockWidget {
        DockWidget::Layout(self)
    }
}

impl Into<DockWidget> for DockItem {
    fn into(self) -> DockWidget {
        DockWidget::Item(self)
    }
}
