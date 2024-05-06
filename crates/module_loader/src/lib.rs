#![allow(unused)]

//! # Expected use cases
//!
//! ```ignore
//! let local_modules = LocalModules::open("/modules_folder");
//!
//! // List installed modules
//! for module_brief in local_modules.summary_modules() {
//!     println!("Module {:?} at {}%", module_brief.id, module_brief.completed_ratio() * 100.);
//! }
//!
//! // List chapters of a module
//! let module = local_modules.load_module(module_id);
//! for book in module.iter_books() {
//!     for chapter in book.iter_chapters() {
//!         println!("Chapter {:?} is {:?}", chapter.id, chapter.completion_status);
//!     }
//! }
//!
//! // Load a chapter solution
//! let solution = chapter.load_solution();
//! ```
//!
#![feature(portable_simd)]

mod manifest;
mod module;

use digolog_math::*;

pub use manifest::*;
pub use module::*;
