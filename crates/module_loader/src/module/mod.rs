mod block;
mod description;
mod id;
mod solution;

use std::{collections::HashMap, path::PathBuf};

pub use block::*;
pub use description::*;
pub use id::*;
pub use solution::*;

pub struct LocalModules {
    modules_path: PathBuf,
    chapters_status: HashMap<ChapterId, ChapterCompletionStatus>,
}

pub struct ModuleSummary {
    pub id: ModuleId,
    pub chapters: u32,
    pub completed_chapters: u32,
}

impl LocalModules {
    pub fn load() -> Self {
        todo!()
    }

    /// Get the title and completion ratio of each local module
    pub fn summary_modules(&self) -> impl Iterator<Item = ModuleSummary> {
        todo!()
    }

    /// Get the books and chapters of one module.
    pub fn load_module() -> Module {
        todo!()
    }

    pub fn load_module_solution() -> ModuleSolution {
        todo!()
    }

    /// If a previous solution exists, then it is loaded and returned.
    /// If not, a new one is created.
    pub fn load_chapter_solution(&self, id: ChapterId) -> ChapterSolution {
        todo!()
    }
}
