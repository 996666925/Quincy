use winit::window::Fullscreen;

pub struct WindowSettings {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub minWidth: i32,
    pub minHeight: i32,
    pub maxWidth: i32,
    pub maxHeight: i32,
    pub fullscreen: Option<Fullscreen>,
    pub resizable: bool,
    pub visible: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Title".to_string(),
            width: 800,
            height: 600,
            minWidth: 100,
            minHeight: 100,
            maxWidth: !1,
            maxHeight: !1,
            fullscreen: None,
            resizable: true,
            visible: true,
        }
    }
}
