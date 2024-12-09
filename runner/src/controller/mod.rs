use crate::{
    bind_group_buffer::{BindGroupBufferType, BufferData, SSBO},
    window::UserEvent,
};
use bytemuck::Zeroable;
use egui::Context;
use egui_winit::winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton},
    event_loop::EventLoopProxy,
};
use glam::*;
use shared::grid::*;
use shared::{particle::*, push_constants::sandsim::ShaderConstants};
use std::time::Instant;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    shader_constants: ShaderConstants,
    grid: Grid<Particle>,
    cursor: Vec2,
    cursor_down: bool,
}

impl Controller {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        let now = Instant::now();
        let grid = Grid::<Particle>::new(size.width as usize, size.height as usize);

        Self {
            size,
            start: now,
            shader_constants: ShaderConstants::zeroed(),
            grid,
            cursor: Vec2::ZERO,
            cursor_down: false,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.cursor_down = match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            };
        }
    }

    pub fn keyboard_input(&mut self, _key: KeyEvent) {}

    pub fn update(&mut self) {
        self.shader_constants = ShaderConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            cursor_down: self.cursor_down.into(),
            cursor: self.cursor.into(),
        }
    }

    pub fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    pub fn ui(
        &mut self,
        _ctx: &Context,
        _ui: &mut egui::Ui,
        _event_proxy: &EventLoopProxy<UserEvent>,
    ) {
    }

    pub fn buffers(&self) -> BufferData {
        BufferData {
            bind_group_buffers: vec![BindGroupBufferType::SSBO(SSBO {
                data: bytemuck::cast_slice(&self.grid.buffer[..]),
                read_only: false,
            })],
        }
    }
}
