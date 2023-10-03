use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
    time::Instant,
};

use egui::{Color32, FontId, Frame, TextStyle, Visuals, Widget};
use egui_glutin_gl::{EguiBackend, EventResponse};
use OvTools::{sync::OnceCell, utils::r#ref::Ref};
use OvWindowing::{event::WindowEvent, event_loop::EventLoop, Window};

use crate::component::Component;

static UIMANAGER: OnceCell<Ref<UiManager>> = OnceCell::new();

pub struct UiManager {
    egui: EguiBackend,
    children: Vec<Ref<dyn Component>>,
    fonts: egui::FontDefinitions,
}

impl Debug for UiManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("UiManager { .. }")
    }
}

impl UiManager {
    pub fn new(window: &Window, el: &EventLoop<()>) -> Ref<UiManager> {
        let egui = EguiBackend::new(window, el);
        egui.egui_ctx.set_visuals(Visuals::light());
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

        let uiManager = Ref::new(Self {
            egui,
            children: Vec::new(),
            fonts: egui::FontDefinitions::default(),
        });
        UIMANAGER.set(uiManager.clone()).unwrap();
        uiManager
    }

    pub fn render(&mut self, window: &Window) {
        let afterTime = self.egui.run(window, |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::none().fill(Color32::TRANSPARENT))
                .show(ctx, |ui| {
                    for comp in &self.children {
                        comp.try_write().unwrap().render(ui, &window);
                    }
                });
        });
       
        self.egui.paint(window);
    }

    pub fn addChild<T>(&mut self, child: &Arc<RwLock<T>>)
    where
        T: Component,
    {
        self.children.push(Ref(child.clone()));
    }

    pub fn handleEvent(&mut self, event: &WindowEvent) -> EventResponse {
        self.egui.on_event(event)
    }
    // pub fn loadFont(&mut self, name: &str) {

    //     self.egui.egui_ctx.set_fonts(font_definitions)
    // }
}
