
use html5ever::{
    local_name, ns,
    tendril::{StrTendril, TendrilSink},
    tree_builder::{
        ElementFlags,
        NodeOrText::{self, AppendNode, AppendText},
        QuirksMode, TreeSink,
    },
    Attribute, ExpandedName, QualName,
};
use html5ever::{namespace_url, parse_document};

use serde::{Deserialize, Serialize};
use QcRender::resources::Resource;
use std::{
    any::Any,
    borrow::Cow,
    mem::size_of,
    ops::{Deref, DerefMut, Index as IndexOps, IndexMut},
    ptr::null,
    sync::Arc,
};
use thunderdome::{Arena, Index};
use QcMacros::UiComp;


use super::{Canvas, DomNode, UiNode};

#[derive(Debug, UiComp, Serialize)]
pub struct HtmlRender {
    document: Index,
    #[serde(skip_serializing)]
    pool: Arena<DomNode>,
    html: String,
}


impl Deref for HtmlRender {
    type Target = Arena<DomNode>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for HtmlRender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

impl IndexOps<Index> for HtmlRender {
    type Output = DomNode;

    fn index(&self, index: Index) -> &Self::Output {
        &self.pool[index]
    }
}
impl IndexMut<Index> for HtmlRender {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.pool[index]
    }
}

impl HtmlRender {
    pub fn new() -> Self {
        let qual_name = QualName::new(None, ns!(), local_name!("root"));

        let doc = DomNode {
            name: qual_name,
            data: "".to_string(),
            children: vec![],
            attr: vec![],
        };

        let mut nodes = Arena::new();
        let document = nodes.insert(doc);
        HtmlRender {
            document,
            pool: nodes,
            html: String::new(),
        }
    }

    pub fn addChild(&mut self, node: DomNode) -> Index {
        self.pool.insert(node)
    }

    pub fn removeChild(&mut self, node: Index) -> Option<DomNode> {
        self.pool.remove(node)
    }

    pub fn loadHtml(html: Resource) -> HtmlRender {
        let canvas = HtmlRender::new();

        let mut canvas = parse_document(canvas, Default::default())
            .from_utf8()
            .one(html.file.data.deref());
        canvas.html = html.name;

        canvas
    }
}

impl TreeSink for HtmlRender {
    type Handle = Index;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        println!("Parse error: {}", msg);
    }

    fn get_document(&mut self) -> Index {
        // unimplemented!()

        self.document
    }

    fn get_template_contents(&mut self, target: &Index) -> Index {
        // if let Some(expanded_name!(html "template")) = self.pool.get(*target).map(|n| n.expanded())
        // {
        //     target + 1
        // } else {

        // }

        *target
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        //println!("Set quirks mode to {:?}", mode);
    }

    fn same_node(&self, x: &Index, y: &Index) -> bool {
        x == y
    }

    fn elem_name(&self, target: &Index) -> ExpandedName {
        self.pool.get(*target).unwrap().name.expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attr: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Index {
        // println!("Created {:?} ", name);

        // if name.local == local_name!("style") {
        //     println!("{:?}", attr);
        // }
        // println!("{:?}", name);
        let id = self.pool.insert(DomNode {
            name,
            data: String::new(),
            children: vec![],
            attr,
        });

        id
    }

    fn create_comment(&mut self, text: StrTendril) -> Index {
        // let id = self.get_id();
        // println!("Created comment \"{}\" as", text.escape_default());
        // id

        self.document
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> Index {
        unimplemented!()
    }

    fn append(&mut self, parent: &Index, child: NodeOrText<Index>) {
        let node = &mut self.pool[*parent];

        match child {
            AppendNode(n) => {
                node.addChild(n);
            }
            AppendText(t) => {
                node.data += &t;
            }
        }
    }

    fn append_before_sibling(&mut self, sibling: &Index, new_node: NodeOrText<Index>) {
        match new_node {
            AppendNode(n) => println!("Append node "),
            AppendText(t) => println!("Append text before  \"{}\"", t.escape_default()),
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        _prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        self.append_before_sibling(element, child);
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        //println!("Append doctype: {} {} {}", name, public_id, system_id);
    }

    fn add_attrs_if_missing(&mut self, target: &Index, attrs: Vec<Attribute>) {
        assert!(self.pool.contains(*target), "not an element");
        //println!("Add missing attributes to {}:", target);
        for attr in attrs.into_iter() {
            //println!("    {:?} = {}", attr.name, attr.value);
        }
    }

    fn associate_with_form(
        &mut self,
        _target: &Index,
        _form: &Index,
        _pool: (&Index, Option<&Index>),
    ) {
        // No form owner support.
    }

    fn remove_from_parent(&mut self, target: &Index) {
        //println!("Remove {} from parent", target);
    }

    fn reparent_children(&mut self, node: &Index, new_parent: &Index) {
        // println!("Move children from {} to {:?}", node, new_parent);
    }

    fn mark_script_already_started(&mut self, node: &Index) {
        println!("Mark script {:?} as already started", node);
    }

    fn set_current_line(&mut self, line_number: u64) {
        // println!("Set current line to {}", line_number);
    }

    fn pop(&mut self, elem: &Index) {
        // println!("Popped element {}", elem);
    }
}

impl<'de> Deserialize<'de> for HtmlRender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(HtmlRender::new())
    }
}
