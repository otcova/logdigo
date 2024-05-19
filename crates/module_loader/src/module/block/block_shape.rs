use crate::*;

/// Instance of a BlockDesc
pub struct BlockShape {
    pub description: BlockDescId,
    pub lable: String,
    pub inputs: Vec<BlockCable>,
    pub outputs: Vec<BlockCable>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct BlockShapeId(u64);

pub struct BlockCable {
    pub lable: String,
    pub wires: u8,
}

impl BlockShape {
    pub fn id(&self) -> BlockShapeId {
        todo!()
    }

    pub fn size(&self) -> Vec2<f32> {
        todo!()
    }
}
