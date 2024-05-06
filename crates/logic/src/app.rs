use crate::*;
use digolog_module_loader::*;
use std::path::Path;

pub struct AppLogic {
    local_modules: LocalModules,
}

impl AppLogic {
    pub fn open(app_folder: impl AsRef<Path>) -> Self {
        let local_modules = LocalModules::open(app_folder.as_ref().join("local_modules"));
        Self { local_modules }
    }
}
