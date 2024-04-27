#![allow(unused)]

//! # Expected use cases
//!
//! ```
//! let local_modules = LocalModules::load();
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

// mod de;
mod math;
mod module;

// pub use de::*;
pub use math::*;
pub use module::*;
