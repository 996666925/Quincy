use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DrawParameters {
    pub cull_face: Option<CullFace>,
    // pub color_write: ColorMask,
    pub depth_write: bool,
    // pub stencil_test: Option<StencilFunc>,
    pub depth_test: bool,
    // pub blend: Option<BlendParameters>,
    // pub stencil_op: StencilOp,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            cull_face: None,
            depth_test: true,
            depth_write: false,
        }
    }
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CullFace {
    Back = gl::BACK,
    Front = gl::FRONT,
}

impl Default for CullFace {
    fn default() -> Self {
        Self::Back
    }
}
