use egui::WidgetText;
use egui_tiles::{Container, Linear, SimplificationOptions, Tabs, TileId, Tiles};

struct Pane {
    name: String,
}

struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
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
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        // let color = egui::epaint::Hsva::new(0.103 * pane.nr as f32, 0.5, 0.5, 1.0);
        // ui.painter().rect_filled(ui.max_rect(), 0.0, color);

        // ui.label(format!("The contents of pane {}.", pane.nr));

        // You can make your pane draggable like so:
        // if ui
        //     .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
        //     .drag_started()
        // {
        //     egui_tiles::UiResponse::DragStarted
        // } else {
        //     egui_tiles::UiResponse::None
        // }
        egui_tiles::UiResponse::None
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 700.0]),
        ..Default::default()
    };

    let mut tree = create_tree();

    eframe::run_simple_native("标题", options, move |ctx, _frame| {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "OPPOSans".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/OPPOSans-R.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(1, "OPPOSans".into());

        ctx.set_fonts(fonts);
        ctx.set_visuals(egui::Visuals::dark()); 

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior {};
            tree.ui(&mut behavior, ui);
        });
    })
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut next_view_nr = 0;

    let mut tiles = egui_tiles::Tiles::default();

    let panel1 = tiles.insert_pane(Pane {
        name: "层级管理器".to_owned(),
    });

    let panel2 = tiles.insert_pane(Pane {
        name: "资源管理器".to_owned(),
    });


    let left = tiles.insert_vertical_tile(vec![panel1, panel2]);

    let panel1 = tiles.insert_pane(Pane {
        name: "场景编辑器".to_owned(),
    });

    let panel2 = tiles.insert_pane(Pane {
        name: "资源预览".to_owned(),
    });

    
    let mut linner = Linear {
        dir: egui_tiles::LinearDir::Vertical,
        children: vec![panel1, panel2],
        ..Default::default()
    };

    linner.shares.set_share(panel1, 0.6);
    linner.shares.set_share(panel2, 0.4);

    let middle = tiles.insert_container(Container::Linear(linner));

    let panel1 = tiles.insert_pane(Pane {
        name: "属性检查器".to_owned(),
    });

    let right = tiles.insert_vertical_tile(vec![panel1]);

    let mut linner = Linear {
        dir: egui_tiles::LinearDir::Horizontal,
        children: vec![left, middle, right],
        ..Default::default()
    };

    linner.shares.set_share(left, 0.2);
    linner.shares.set_share(middle, 0.6);
    linner.shares.set_share(right, 0.2);

    let container = tiles.insert_container(Container::Linear(linner));



    egui_tiles::Tree::new("my_tree", container, tiles)
}
