mod builtin;

use crate::*;
use std::path::*;

impl Module {
    pub fn from_path(path: impl AsRef<Path>) -> u8 {
        println!("Build Time! :)");
        123
    }

    // pub fn builtin_mods() -> Vec<Self> {
    //     vec![]
    // }
}
