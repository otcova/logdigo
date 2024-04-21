use crate::*;

pub struct App {
    modules: Vec<Module>,
}

impl App {
    // pub fn new() -> Self {
    //     Self {
    //         modules: Module::builtin_mods(),
    //     }
    // }

    pub fn iter_modules(&self) -> impl Iterator<Item = &Module> {
        self.modules.iter()
    }
}
