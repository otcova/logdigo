//! # What is digilog_app
//! This crate is responsable of:
//!  - Managing the window and graphics
//!  - Implement a UI layer specific for Digolog
//!  - Giving an interface to use the library
//!
//! # Expected use case
//! ```ignore
//! use digolog_app::*;
//!
//! fn main() {
//!     run_app(Digolog::setup());
//! }
//!
//! struct Digolog { /* data */ }
//!
//! impl Digolog {
//!     async fn setup() -> Self {
//!         // Load App data in parallel of the window opening.
//!         Digolog { /* data */ }
//!     }
//! }
//!
//! impl AppBrain for Digolog {
//!
//!     fn init(&mut self, ui: &mut AppUI) {
//!         // The window is ready to display.
//!         // Create all the initial ui objects.
//!
//!         let panel = ui.create_panel(PanelDescriptor { .. });
//!         let block = ui.create_block(BlockDescriptor { .. });
//!         let wire = ui.create_wire(WireDescriptor { .. });
//!
//!         // ui objects are deleted on drop.
//!     }
//! }
//! ```

#![allow(unused, incomplete_features)]
#![feature(generic_nonzero, generic_const_exprs, slice_range)]

mod app;
mod app_handle;
mod graphics;

use app::*;
use app_handle::*;
use async_std::task;
use std::future::Future;
use winit::event_loop::EventLoop;

// pub use app::ui;
// pub use app::ui::UI;
pub use app::AppBrain;
pub use digolog_math::*;

pub fn run_app<B: AppBrain>(brain: impl Future<Output = B> + Send + 'static) {
    let app = LoadingAppHandle {
        brain: task::spawn(brain),
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut AppHandle::Loading(app)).unwrap();
}
