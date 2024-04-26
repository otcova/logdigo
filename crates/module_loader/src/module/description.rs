use crate::*;
use std::collections::HashMap;

pub struct Module {
    pub id: ModuleId,
    pub description: String,
    pub author: Vec<String>,
    pub blocks: HashMap<String, BlockDesc>,
    pub books: HashMap<String, Book>,
}

pub struct Book {
    pub chapters: Vec<Chapter>,
}

pub struct Chapter {
    pub title: String,
    pub allowed_blocks: Vec<BlockDescId>,
}
