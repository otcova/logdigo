use crate::*;

pub struct ModuleSummary {
    pub id: ModuleId,
    pub chapters: u32,
    pub completed_chapters: u32,
}

impl ModuleSummary {
    pub fn list_modules() -> impl Iterator<Item = ModuleSummary> {
        todo!()
    }
}
