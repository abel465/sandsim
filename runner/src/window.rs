use crate::shader::CompiledShaderModules;
use egui_winit::winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::wayland::*,
    window::{self},
};

pub enum UserEvent {
    NewModule(CompiledShaderModules),
    SetVSync(bool),
}

pub struct Window {
    pub event_loop: EventLoop<UserEvent>,
    pub window: window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let window_attributes = egui_winit::winit::window::Window::default_attributes()
            .with_title("sandsim")
            .with_name("sandsim", "")
            .with_inner_size(PhysicalSize::new(1280.0, 720.0));

        let window = event_loop.create_window(window_attributes).unwrap();

        Self { event_loop, window }
    }
}
