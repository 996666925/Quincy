use html5ever::{Attribute, QualName};
use thunderdome::Index;

#[derive(Debug)]
pub struct DomNode {
   pub name: QualName,
   pub data: String,
   pub attr: Vec<Attribute>,
   pub children: Vec<Index>,
}

impl DomNode {
    pub fn addChild(&mut self, child: Index) {
        self.children.push(child);
    }
}
