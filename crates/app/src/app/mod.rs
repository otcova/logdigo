mod brain;
mod renderer;

use async_std::task;
use renderer::*;
use std::sync::Arc;
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

pub use brain::*;

pub struct App<B: AppBrain> {
    brain: B,
    renderer: Renderer,
}

impl<B: AppBrain> App<B> {
    pub async fn new(window: Window, brain: task::JoinHandle<B>) -> Self {
        let window = Arc::new(window);
        let renderer = Renderer::new(window).await;

        let mut brain = brain.await;
        brain.init();

        Self { brain, renderer }
    }

    pub fn input(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.renderer.resize(new_size);
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
                self.renderer.render().unwrap();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.renderer.clear_color = wgpu::Color {
                    r: position.x as f64 / self.renderer.size().width as f64,
                    g: position.y as f64 / self.renderer.size().height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                self.renderer.window.request_redraw();
            }
            _ => (),
        }
    }
}
