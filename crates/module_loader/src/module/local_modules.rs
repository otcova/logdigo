use crate::*;
use std::collections::HashMap;
use std::path::*;

pub struct LocalModules {
    modules_folder: PathBuf,
}

pub struct ModuleSummary {
    pub id: ModuleId,
    pub chapters: u32,
    pub completed_chapters: u32,
}

impl LocalModules {
    pub fn open(modules_folder: PathBuf) -> Self {
        Self { modules_folder }
    }

    // /// Get the title and completion ratio of each local module
    // pub fn summary_modules(&self) -> impl Iterator<Item = ModuleSummary> {
    //     todo!()
    // }

    /// Get the books and chapters of one module.
    pub fn load_module() -> Module {
        todo!()
    }
}
