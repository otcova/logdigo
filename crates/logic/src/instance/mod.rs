use crate::*;
use digolog_module_loader::Vec2;

pub struct Module {
    pub info: ModuleInfo,
    pub books: Vec<Book>,
}

pub struct Book {
    pub info: BookInfo,
    pub chapters: Vec<Chapter>,
}

pub struct Chapter {
    pub info: ChapterInfo,
    pub blocks: Vec<Block>,
}

/// Instance of a Block on a given position
pub struct Block {
    // TODO:[Perfomance] Share Block struct for multipl BlockPanel instances. (Change 'block' to be an index or a Rc<>)
    pub shape: BlockShape,
    pub pos: Vec2,
}

/// Index of a Block in 'Chapter::blocks'
pub struct BlockId(usize);

/// Instance of a BlockTemplate
pub struct BlockShape {
    pub lable: String,
    pub inputs: Vec<BlockCable>,
    pub outputs: Vec<BlockCable>,
}

pub struct BlockCable {
    pub lable: String,
    pub wires: u8,
}
