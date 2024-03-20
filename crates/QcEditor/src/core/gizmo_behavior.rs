use QcTools::utils::r#ref::Ref;

pub enum GizmoOperation {
    Translate,
    Rotate,
    Scale,
}

pub enum Direction {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct GizmoBehavior {}

impl GizmoBehavior {
    pub fn new() -> Ref<Self> {
        let this = Self {};
        Ref::new(this)
    }

    pub fn start_picking(&self) {}

    pub fn stop_picking(&self) {}
}
