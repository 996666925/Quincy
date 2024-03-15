use std::sync::mpsc::Sender;

use egui::{Color32, Margin, Vec2};

use QcUI::{
    component::{Button, ButtonMessage, Canvas, Label, Panel, PanelWindow, TextBox, ToUi, Widget},
    prelude::FlexDirection,
};

use crate::core::message::Page;

use crate::core::{message::EditorMessage, project::ProjectConfig};

pub struct ProjectHubPage {
    canvas: Canvas,
    sender: Sender<EditorMessage>,
}

impl ProjectHubPage {
    pub fn new(sender: Sender<EditorMessage>) -> Self {
        let mut hub = ProjectHubPage {
            canvas: Canvas::new(),
            sender,
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        let sender = self.sender.clone();

        let projects = vec![
            ProjectConfig::new("Test1", "c:/"),
            ProjectConfig::new("Test2", "c:/Test2"),
        ];

        let mut list = vec![];

        for project in projects {
            let item = Panel::new(Widget::default().with_margin(Margin {
                top: 20.,
                ..Default::default()
            }))
            .with_spacing(20.)
            .with_children(vec![
                Label::new(Widget::default().with_width(600.))
                    .with_color(Color32::WHITE)
                    .with_text(&project.path)
                    .toUi(),
                Button::new(
                    Widget::default()
                        .with_background(Color32::from_rgb(179, 128, 0))
                        .with_height(20.)
                        .with_width(50.),
                )
                .with_text("打开")
                .toUi(),
                Button::new(
                    Widget::default()
                        .with_background(Color32::from_rgb(128, 0, 0))
                        .with_height(20.)
                        .with_width(50.),
                )
                .with_text("删除")
                .toUi(),
            ]);

            list.push(item.toUi());
        }

        Panel::new(
            Widget::default()
                .with_padding(100f32.into())
                .with_background(Color32::from_rgb(23, 23, 26)),
        )
        .with_orientation(FlexDirection::Column)
        .with_children(vec![
            Panel::default()
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
                                Box::new(move |msg| {
                                    println!("打开项目");

                                    sender.send(EditorMessage::GoTo(Page::Editor)).unwrap();
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
                .toUi(),
            Panel::default()
                .with_orientation(FlexDirection::Column)
                .with_children(list)
                .toUi(),
        ])
        .build(&mut self.canvas);
    }
}

impl PanelWindow for ProjectHubPage {
    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    fn get_size(&self) -> Vec2 {
        Vec2::new(1000., 580.)
    }
}
