use crate::*;
use async_std::task;
use derive_more::From;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(From, Default)]
pub enum AppHandle<B: AppBrain> {
    #[default]
    Empty,
    Loading(LoadingAppHandle<B>),
    Running(App<B>),
}

pub struct LoadingAppHandle<B: AppBrain> {
    pub brain: task::JoinHandle<B>,
}

impl<B: AppBrain> ApplicationHandler for AppHandle<B> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Loading(loading_app) = std::mem::take(self) else {
            panic!("Invalid AppHandle stage. Expected Loading");
        };

        let window_attributes = Window::default_attributes().with_title("Digolog");
        let window = event_loop.create_window(window_attributes).unwrap();

        let app = App::new(window, loading_app.brain);
        *self = Self::Running(task::block_on(app));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Self::Running(app) = self else {
            panic!("Invalid AppHandle stage. Expected Running");
        };

        app.input(event_loop, event);
    }
}
