use crate::*;

pub struct BlockDesc {
    pub id: BlockDescId,
    pub lable: String,
    pub group: String,
    pub inputs: Vec<BlockPinDesc>,
    pub outputs: Vec<BlockPinDesc>,
    pub logic: Option<BlockLogic>,
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
