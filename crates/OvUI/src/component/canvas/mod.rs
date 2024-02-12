

use egui::Ui;
use html5ever::{Attribute, QualName};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use thunderdome::{Arena, Index};
use OvMacros::Comp;
use OvWindowing::Window;

mod canvas;
mod uiNode;

pub use canvas::*;
pub use uiNode::*;

