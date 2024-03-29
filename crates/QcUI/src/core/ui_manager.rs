use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock,
    },
    time::Instant,
};

use deno_core::{
    serde_v8,
    v8::{self, HandleScope},
};
use egui::{Color32, FontId, Frame, TextStyle, Vec2, Visuals, Widget};
use egui_glutin_gl::{EguiBackend, EventResponse};

use enum_variant_eq::EnumVariantEq;
use serde::Serialize;
use thunderdome::Index;
use QcCore::scene_system::scene::Scene;
use QcTools::{message::messageSender::MessageSender, sync::OnceCell, utils::r#ref::Ref};
use QcWindowing::{event::WindowEvent, event_loop::EventLoop, window, Window};

use crate::{
    component::{ButtonMessage, Canvas, Component},
    message::{UiMessage, UiMessageType},
};

use super::{context::UiContext, uiBind::UiBind};

pub struct UiManager {
    egui: EguiBackend,
    fonts: egui::FontDefinitions,

    receiver: Receiver<UiMessage>,
    sender: MessageSender<UiMessage>,
}

pub static DEFAULT_FONT: &str = "OPPOSans";

impl Debug for UiManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("UiManager { .. }")
    }
}

impl UiManager {
    pub fn new(window: &Window, el: &EventLoop<()>) -> Ref<UiManager> {
        let mut egui = EguiBackend::new(window, el);
        Self::init(&mut egui);
        let (sender, receiver) = channel();

        let uiManager = Ref::new(Self {
            egui,
            fonts: egui::FontDefinitions::default(),
            sender: MessageSender::new(sender),
            receiver,
        });

        uiManager
    }

    fn init(egui: &mut EguiBackend) {
        let mut visuals = Visuals::light();

        visuals.widgets.hovered.expansion = 0.;
        visuals.widgets.active.expansion = 0.;

        egui.egui_ctx.set_visuals(visuals);
        egui.egui_ctx.style_mut(|style| {
            style.spacing.button_padding = Vec2::ZERO;
            style.spacing.item_spacing = Vec2::ZERO;
        });
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "OPPOSans".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/OPPOSans-R.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(1, "OPPOSans".into());

        egui.egui_ctx.set_fonts(fonts);
    }

    pub fn render(&mut self, window: &Window, canvasList: &mut Vec<&mut Canvas>) {
        let _ = self.egui.run(window, |ctx| {
            for canvas in &mut *canvasList {
                egui::CentralPanel::default()
                    .frame(Frame::none().fill(Color32::TRANSPARENT))
                    .show(ctx, |ui| {
                        for (_, comp) in canvas.iter_mut() {
                            comp.value.renderTop(&mut UiContext::new(ui, &self.sender));
                        }
                    });
            }
        });

        self.egui.paint(window);
    }

    pub fn update(&mut self, canvasList: Vec<&mut Canvas>, scope: &mut HandleScope) {
        let canvasList = canvasList
            .iter()
            .filter(|canvas| !canvas.uiBindList.is_empty())
            .collect::<Vec<&&mut Canvas>>();

        if canvasList.is_empty() {
            return;
        }
        while let Ok(msg) = self.receiver.try_recv() {
            let id = msg.0;

            let msg = msg.1;

            for canvas in &canvasList {
                let uiBind = canvas.getUiBind(id);
                if let Some(bind) = uiBind {
                    let list = bind
                        .iter()
                        .filter(|u| u.get_msg_type().eq(&msg))
                        .collect::<Vec<&UiBind>>();

                    if list.is_empty() {
                        return;
                    }

                    Self::postUiMessage(scope, &list, &msg);
                }
            }
        }
    }

    pub fn update_not_js(&mut self, canvasList: Vec<&mut Canvas>) {
        let canvasList = canvasList
            .iter()
            .filter(|canvas| !canvas.uiBindList.is_empty())
            .collect::<Vec<&&mut Canvas>>();

        if canvasList.is_empty() {
            return;
        }
        while let Ok(msg) = self.receiver.try_recv() {
            let id = msg.0;

            let msg = msg.1;

            for canvas in &canvasList {
                let uiBind = canvas.getUiBind(id);
                if let Some(bind) = uiBind {
                    let list = bind
                        .iter()
                        .filter(|u| u.get_msg_type().eq(&msg))
                        .collect::<Vec<&UiBind>>();

                    if list.is_empty() {
                        return;
                    }

                    for e in list {
                        match e {
                            UiBind::NativeBind(bind) => bind.call(),
                            UiBind::JsBind(_) => {}
                        }
                    }
                }
            }
        }
    }

    pub fn handleEvent(&mut self, window: &Window, event: &WindowEvent) -> EventResponse {
        self.egui.on_window_event(window, event)
    }

    pub fn destory(&mut self) {
        self.egui.destroy();
    }
    // pub fn loadFont(&mut self, name: &str) {

    //     self.egui.egui_ctx.set_fonts(font_definitions)
    // }

    pub fn postUiMessage(scope: &mut v8::HandleScope, bind: &Vec<&UiBind>, data: &impl Serialize) {
        let context = scope.get_current_context();

        let global = context.global(scope);
        let funcName = v8::String::new(scope, "__POST_MESSAGE__").unwrap();

        let func = global.get(scope, funcName.into()).unwrap();

        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let args = serde_v8::to_v8(scope, data).unwrap();

        let typeName = serde_v8::to_v8(scope, "ui").unwrap();
        let undefined = v8::undefined(scope).into();
        let uiBind = serde_v8::to_v8(scope, bind).unwrap();
        func.call(scope, undefined, &[typeName, args, uiBind]);
    }

    pub fn defalut_font(&self) -> &str {
        DEFAULT_FONT
    }
}
