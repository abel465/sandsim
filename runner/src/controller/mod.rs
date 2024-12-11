use crate::{
    bind_group_buffer::{BindGroupBufferType, BufferData, SSBO},
    user_event::UserEvent,
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
    prev_cursor: Vec2,
    cursor_down: bool,
    cursor_right_down: bool,
    current_particle_type: ParticleType,
    brush_size: f32,
    offset: u32,
}

impl Controller {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        let now = Instant::now();
        let grid = Grid::<Particle>::from_fn(size.width as usize, size.height as usize, |_, _| {
            Particle::default()
        });

        Self {
            size,
            start: now,
            shader_constants: ShaderConstants::zeroed(),
            grid,
            cursor: Vec2::ZERO,
            prev_cursor: Vec2::ZERO,
            cursor_down: false,
            cursor_right_down: false,
            current_particle_type: ParticleType::Sand,
            brush_size: 20.0,
            offset: 0,
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
        } else if button == MouseButton::Right {
            self.cursor_right_down = match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            };
        }
    }

    pub fn keyboard_input(&mut self, _key: KeyEvent) {}

    pub fn update(&mut self) {
        let particle_type = if self.cursor_right_down {
            ParticleType::Empty
        } else {
            self.current_particle_type
        };
        self.shader_constants = ShaderConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            cursor_down: (self.cursor_down || self.cursor_right_down).into(),
            cursor: self.cursor.into(),
            prev_cursor: self.prev_cursor.into(),
            current_particle_type: particle_type as u32,
            brush_size_sq: self.brush_size * self.brush_size,
            offset: self.offset,
        };
        self.prev_cursor = self.cursor;
        self.offset = 1 - self.offset;
    }

    pub fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    pub fn ui(
        &mut self,
        _ctx: &Context,
        ui: &mut egui::Ui,
        _event_proxy: &EventLoopProxy<UserEvent>,
    ) {
        ui.radio_value(&mut self.current_particle_type, ParticleType::Sand, "Sand");
        ui.radio_value(
            &mut self.current_particle_type,
            ParticleType::Water,
            "Water",
        );
        ui.add(egui::Label::new("       Brush Size").selectable(false));
        ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=1000.0).logarithmic(true));
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
