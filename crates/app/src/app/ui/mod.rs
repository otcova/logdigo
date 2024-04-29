mod object;
mod painter;

use super::Renderer;
use winit::dpi::PhysicalSize;

pub use object::*;
use painter::*;

pub use wgpu::Color;

/// Drawing interface for the AppBrain.
pub struct UI {
    renderer: Renderer,
    painters: Painters,
}

impl UI {
    pub(crate) fn new(renderer: Renderer) -> Self {
        Self {
            painters: Painters::new(&renderer),
            renderer,
        }
    }

    pub(crate) fn render(&mut self) {
        let mut encoder = self.renderer.create_encoder();
        self.painters.render(&mut encoder, &self.renderer);
        self.renderer.render(encoder);
    }
    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub(crate) fn request_redraw(&self) {
        self.renderer.window.request_redraw();
    }
}
