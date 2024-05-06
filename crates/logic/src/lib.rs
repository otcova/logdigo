//! # What is digolog-logic
//! This crate has all the app core logic of digolog. This does not include
//! the rendering and ui.
//!
//! # Expected use cases:
//! ```
//! use digolog_logic::*;
//! let logic = AppLogic::open("/digolog_folder");
//! ```

#![allow(unused)]

mod app;
// mod modules;
// mod runner;

pub use app::*;
// pub use modules::*;
// pub use runner::*;
