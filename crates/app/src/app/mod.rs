mod brain;
mod renderer;
pub mod ui;

use async_std::task;
use std::sync::Arc;
use ui::*;
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

pub use brain::*;
pub use renderer::*;
pub use ui::*;

/// Responsible of managing the `AppBrain`
pub struct App<B: AppBrain> {
    brain: B,
    ui: UI,
}

impl<B: AppBrain> App<B> {
    pub async fn new(window: Window, brain: task::JoinHandle<B>) -> Self {
        let window = Arc::new(window);
        let renderer = Renderer::new(window).await;
        let mut ui = UI::new(renderer);

        let mut brain = brain.await;
        brain.init(&mut ui);

        Self { brain, ui }
    }

    pub fn input(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.ui.resize(new_size);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                Key::Named(NamedKey::Escape) => {
                    event_loop.exit();
                }
                _ => (),
            },
            WindowEvent::RedrawRequested => {
                self.ui.render();
            }
            WindowEvent::CursorMoved { .. } => {
                // ...
            }
            _ => (),
        }
    }
}
