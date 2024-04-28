pub mod object;

use super::Renderer;
use winit::dpi::PhysicalSize;

pub use object::*;
pub use wgpu::Color;

/// Drawing interface for the AppBrain.
pub struct UI {
    renderer: Renderer,
    objects: Objects,
}

impl UI {
    pub(crate) fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
            objects: Objects::default(),
        }
    }

    pub(crate) fn render(&mut self) {
        self.renderer.render().unwrap();
    }
    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub(crate) fn request_redraw(&self) {
        self.renderer.window.request_redraw();
    }
}
