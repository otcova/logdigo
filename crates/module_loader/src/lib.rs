#![allow(unused)]

//! # Expected use cases
//!
//! ## Get local (installed) modules
//! It will read files and cache some information.
//! This should be done once per aplication.
//! ```
//! let local_modules = LocalModules::load();
//! local_modules.summary_modules(); // List completed and installed modules
//! let module = local_modules.load_module(module_id); // Check the module books and chapters
//! module.chapter_status(chapter_id); // Check the completion status of a chapter
//! local_modules.load_chapter_solution(chapter_id); // Open a solution to edit and run
//! ```
//!

mod de;
mod error;
mod math;
mod module;

pub use de::*;
pub use error::*;
pub use math::*;
pub use module::*;
