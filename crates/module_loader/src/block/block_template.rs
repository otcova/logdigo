use crate::*;

pub struct BlockTemplate {
    pub name: String,
    pub lable: String,
    pub group: String,
    pub inputs: Vec<BlockPinTemplate>,
    pub outputs: Vec<BlockPinTemplate>,
    pub logic: Option<BlockLogic>,
}

pub struct BlockPinTemplate {
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
