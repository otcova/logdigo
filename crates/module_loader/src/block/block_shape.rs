use crate::*;

/// Instance of a BlockTemplate
pub struct BlockShape {
    pub lable: String,
    pub template: BlockTemplateId,
    pub inputs: Vec<BlockCable>,
    pub outputs: Vec<BlockCable>,
}

pub struct BlockCable {
    pub lable: String,
    pub wires: u8,
}
