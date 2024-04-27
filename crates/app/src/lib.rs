//! # What is digilog-app
//! This module is responsable of:
//!  - Managing the window and graphics
//!  - Implement a UI layer specific for Digolog
//!  - Giving an interface to use the library
//!
//! # Expected use case
//! ```
//! fn main() {
//!     run_app(Digolog::setup());
//! }
//!
//! struct Digolog { /* data */ }
//!
//! impl Digolog {
//!     async fn setup() -> Self {
//!         // Load App data in parallel of the window opening.
//!     }
//! }
//!
//! impl AppBrain for Digolog {
//!
//!     fn init(&mut self, ui: &mut AppUI) {
//!         // The window is ready to display.
//!         // Create all the initial ui objects.
//!
//!         let left_panel = ui.create(LeftPanelInfo);
//!         let block = ui.create(BlockInfo);
//!         let wire = ui.create(WireInfo);
//!
//!         ui.delete(left_panel);
//!         ui.delete(block);
//!         ui.delete(wire);
//!     }
//! }
//! ```

mod app;
mod app_handle;

use app::*;
use app_handle::*;
use async_std::task;
use std::future::Future;
use winit::event_loop::EventLoop;

pub use app::AppBrain;

pub fn run_app<B: AppBrain>(brain: impl Future<Output = B> + Send + 'static) {
    let app = LoadingAppHandle {
        brain: task::spawn(brain),
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut AppHandle::Loading(app)).unwrap();
}
