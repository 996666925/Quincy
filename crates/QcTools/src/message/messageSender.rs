use std::sync::mpsc::Sender;
#[derive(Debug, Clone)]
pub struct MessageSender<T>(Sender<T>);

impl<T> MessageSender<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self(sender)
    }
    pub fn sendMessage(&self, msg: T) {
        self.0.send(msg).unwrap();
    }
}
