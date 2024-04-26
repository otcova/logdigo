use std::{collections::HashMap, path::PathBuf};

use crate::*;

pub struct ModuleSolution {
    pub id: ModuleId,
    books: HashMap<String, BookSolution>,
}

pub struct BookSolution {
    pub id: BookId,
    chapters: HashMap<String, ChapterSolutionInfo>,
}

#[derive(Clone)]
struct ChapterSolutionInfo {
    save_file_path: PathBuf,
    pub completion_status: ChapterCompletionStatus,
}

#[derive(Debug, Copy, Clone)]
pub enum ChapterCompletionStatus {
    NotStarted,
    InProgress,
    Completed,
}

pub struct ChapterSolution {
    pub info: ChapterSolutionInfo,
    pub blocks: Vec<Block>,
}

impl BookSolution {
    pub fn get_chapter(&self, chapter_id: &ChapterId) -> &ChapterSolutionInfo {
        if self.id != chapter_id.book_id {
            panic!("Invalid ChapterId. Requesting a chapter of a diferent module");
        }
        &self.chapters[&chapter_id.title]
    }
}

impl ChapterSolutionInfo {
    pub fn load_solution(&self, chapter_desc: &ChapterDesc) -> ChapterSolution {
        let save_file = match load_solution_save_file(self.save_file_path) {
            Some(f) => f,
            None => chapter_desc.solution_template(),
        };
        ChapterSolution {
            blocks: save_file.blocks,
            info: self.clone(),
        }
    }
}

impl ModuleSolution {
    pub fn get_book(&self, book_id: &BookId) -> &BookSolution {
        if self.id != book_id.module_id {
            panic!("Invalid BookId. Requesting a book of a diferent module");
        }
        &self.books[&book_id.title]
    }
}
