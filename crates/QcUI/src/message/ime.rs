use serde::{Deserialize, Serialize};
use enum_variant_eq::{*,enum_variant_eq_derive::*};

#[derive(Debug, Deserialize, Serialize, Clone, EnumVariantEq)]
pub enum ImeMessage {
    ImeStart,
    ImeStop,
}
