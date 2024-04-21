mod block_logic;

use crate::*;

pub fn get_all() -> Vec<Module> {
    vec![fundamentals()]
}

pub fn fundamentals() -> Module {
    Module {
        info: ModuleInfo {
            author: vec!["Otger".into()],
            title: "Fundamentals".into(),
            description: "".into(),
            used_blocks: vec![BlockTemplate {
                name: "And".into(),
                lable: "&".into(),
                logic: Some(BlockLogic::Builtin(block_logic::and)),
                inputs: vec![],
                outputs: vec![],
            }],
        },
        books: vec![],
    }
}
