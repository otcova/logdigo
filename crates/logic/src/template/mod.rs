use crate::*;

/// The "author[0]/title" of a book.
/// In case of a builtin book it is "title"
pub struct ModuleTemplateId(String);

pub struct ModuleInfo {
    /// In case of a group of authors, author[0] should be:
    ///     - The principal creator or founder
    /// This is because "author[0]" is used as part of the primary key.
    pub author: Vec<String>,
    pub title: String,
    pub description: String,
    pub used_blocks: Vec<BlockTemplate>,
}

pub struct BookInfo {
    pub title: String,
}

pub struct ChapterInfo {
    pub title: String,
    pub allowed_blocks: Vec<BlockTemplateId>,
}

/// Index of `ModuleInfo::used_blocks`
pub struct BlockTemplateId(usize);

pub struct BlockTemplate {
    pub name: String,
    pub lable: String,
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
