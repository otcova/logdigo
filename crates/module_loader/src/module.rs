use crate::*;

pub struct Module {
    pub name: String,
    pub description: String,
    pub author: Vec<String>,
    pub blocks: Vec<BlockTemplate>,
    pub books: Vec<Book>,
}

/// Index of `Module::used_blocks`
pub struct BlockTemplateId(pub(crate) usize);

pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

pub struct Chapter {
    pub title: String,
    pub allowed_blocks: Vec<BlockTemplateId>,
    pub blocks: Vec<Block>,
}
