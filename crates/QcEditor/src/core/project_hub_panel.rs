use egui::{Color32, Margin};
use env_logger::fmt::style::Color;
use QcUI::{
    component::{
        Button, ButtonMessage, Canvas, Panel, PanelWindow, TextBox, ToUi, UiNodeTrait, Widget,
    },
    core::ui_manager::UiManager,
    prelude::FlexDirection,
};

pub struct ProjectHubPanel {
    canvas: Canvas,
}

impl ProjectHubPanel {
    pub fn new() -> Self {
        let mut hub = ProjectHubPanel {
            canvas: Canvas::new(),
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        Panel::new(
            Widget::default()
                .with_padding(100f32.into())
                .with_background(Color32::from_rgb(23, 23, 26)),
        )
        .with_children(vec![Panel::default()
            .with_orientation(FlexDirection::Row)
            .with_spacing(20.)
            .with_children(vec![
                Button::new(
                    Widget::default()
                        .with_background(Color32::from_rgb(179, 128, 0))
                        .with_height(30.)
                        .with_width(100.)
                        .on_event(
                            ButtonMessage::Clicked.into(),
                            Box::new(|msg| {
                                println!("打开项目");
                            }),
                        )
                        .build(&mut self.canvas),
                )
                .with_text("打开项目")
                .toUi(),
                Button::new(
                    Widget::default()
                        .with_background(Color32::from_rgb(0, 128, 0))
                        .with_height(30.)
                        .with_width(100.),
                )
                .with_text("新建项目")
                .toUi(),
                TextBox::new(
                    Widget::default()
                        .with_width(500.)
                        .with_height(30.)
                        .with_padding(5f32.into()),
                )
                .with_hint_text("输入项目路径")
                .toUi(),
                Button::new(
                    Widget::default()
                        .with_background(Color32::from_rgb(0, 128, 0))
                        .with_height(30.)
                        .with_width(30.),
                )
                .with_text("GO")
                .toUi(),
            ])
            .toUi()])
        .build(&mut self.canvas);
    }
}

impl PanelWindow for ProjectHubPanel {
    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }
}
