use crate::*;
use digolog_module_loader::*;
use std::path::PathBuf;

pub struct AppLogic {
    local_modules: LocalModules,
}

impl AppLogic {
    pub fn load(app_folder: PathBuf) -> Self {
        let local_modules = LocalModules::load(app_folder.join("local_modules"));
        Self { local_modules }
    }
}
