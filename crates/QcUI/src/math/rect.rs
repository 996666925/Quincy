use egui::Rect;
use QcWindowing::dpi::PhysicalSize;
#[derive(Debug)]
pub struct QcRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl QcRect {
    pub fn to_gl_rect(rect: Rect, size: PhysicalSize<u32>, scale: f32) -> Self {
        Self {
            x: (rect.min.x * scale).ceil() as _,
            y: (size.height as f32 - rect.max.y * scale).ceil() as _,
            width: (rect.width() * scale) as _,
            height: (rect.height() * scale) as _,
        }
    }
}

impl From<Rect> for QcRect {
    fn from(value: Rect) -> Self {
        Self {
            x: value.min.x as _,
            y: value.max.y as _,
            width: value.width() as _,
            height: value.height() as _,
        }
    }
}
