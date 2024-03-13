pub enum EditorMessage {
    GoTo(Page),
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Page {
    ProjectHub,
    Editor,
}
