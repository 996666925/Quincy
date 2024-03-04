

use egui::Ui;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use thunderdome::{Arena, Index};
use QcMacros::Comp;
use QcWindowing::Window;

mod canvas;
mod uiNode;

pub use canvas::*;
pub use uiNode::*;

