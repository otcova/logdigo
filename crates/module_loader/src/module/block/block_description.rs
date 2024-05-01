use crate::*;
use std::sync::Arc;

/// Represents all the shapes that a block can have
pub struct BlockDesc {
    pub id: BlockDescId,
    pub lable: String,
    pub group: String,
    pub color: Color,
    pub inputs: Vec<BlockPinDesc>,
    pub outputs: Vec<BlockPinDesc>,
    pub logic: Option<BlockLogic>,
}

/// A subset of all the block shapes that a BlockDesc represents
pub struct BlockDescSubset {
    pub block_desc: Arc<BlockDesc>,
    // TODO: Represent the subset
}

pub struct BlockPinDesc {
    pub pin_type: PinTypeTemplate,
    pub lable: String,
}
pub enum PinTypeTemplate {
    Bundle {
        cables: TemplateNumber,
        wires_per_cable: TemplateNumber,
    },
    Cable {
        wires: TemplateNumber,
    },
}

pub enum TemplateNumber {
    Const(String),
    Num(i64),
}
