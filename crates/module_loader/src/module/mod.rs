mod block;
mod id;
mod local_modules;
mod solution;

pub use block::*;
pub use id::*;
pub use local_modules::*;
pub use solution::*;

use std::collections::HashMap;

pub struct Module {
    pub id: ModuleId,
    pub description: String,
    pub author: Vec<String>,
    pub blocks: HashMap<String, BlockDesc>,
    books: HashMap<String, Book>,
}

pub struct Book {
    pub id: BookId,
    chapters: Vec<Chapter>,
}

pub struct Chapter {
    pub id: ChapterId,
    pub completion_status: ChapterCompletionStatus,
    pub allowed_blocks: Vec<BlockDescId>,
}

impl Module {
    pub fn get_book(&self, book_id: &BookId) -> &Book {
        if self.id != book_id.module_id {
            panic!("Invalid BookId. Requesting a book of a diferent module");
        }
        &self.books[&book_id.title]
    }
    pub fn iter_books(&self) -> impl Iterator<Item = &Book> {
        self.books.values()
    }
}

impl Book {
    pub fn iter_chapters(&self) -> impl Iterator<Item = &Chapter> {
        self.chapters.iter()
    }
}
