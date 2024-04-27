use crate::*;

/// Instance of a BlockDesc
pub struct BlockShape {
    pub description: BlockDescId,
    pub lable: String,
    pub inputs: Vec<BlockCable>,
    pub outputs: Vec<BlockCable>,
}

pub struct BlockCable {
    pub lable: String,
    pub wires: u8,
}
