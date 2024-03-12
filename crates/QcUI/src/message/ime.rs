use enum_variant_eq::{enum_variant_eq_derive::*, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, EnumVariantEq)]
pub enum ImeMessage {
    ImeStart,
    ImeStop,
}
