use crate::{
    bind_group_buffer::{BindGroupBufferType, BufferData, SSBO},
    user_event::UserEvent,
};
use bytemuck::Zeroable;
use egui::Context;
use egui_winit::winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    event_loop::EventLoopProxy,
    keyboard::Key,
};
use glam::*;
use shared::grid::*;
use shared::{particle::*, push_constants::sandsim::*};
use std::time::Instant;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    fragment_constants: FragmentConstants,
    compute_constants: ComputeConstants,
    grid: Grid<Particle>,
    cursor: Vec2,
    prev_cursor: Vec2,
    cursor_down: bool,
    cursor_right_down: bool,
    current_particle_type: ParticleType,
    brush_size: f32,
    offset: u32,
    speed: f32,
    distance: f32,
    last_frame: Instant,
    zoom: f32,
    debug: bool,
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
            fragment_constants: FragmentConstants::zeroed(),
            compute_constants: ComputeConstants::zeroed(),
            grid,
            cursor: Vec2::ZERO,
            prev_cursor: Vec2::ZERO,
            cursor_down: false,
            cursor_right_down: false,
            current_particle_type: ParticleType::Sand,
            brush_size: 20.0,
            offset: 0,
            speed: normalize_speed_down(1.0),
            distance: 0.0,
            last_frame: now,
            zoom: 1.0,
            debug: false,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
    }

    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        let val = match delta {
            MouseScrollDelta::LineDelta(_, val) => val * 0.1,
            MouseScrollDelta::PixelDelta(p) => (p.y * 0.005) as f32,
        };
        self.zoom = (self.zoom + self.zoom * val).clamp(1.0, 100.0);
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

    pub fn keyboard_input(&mut self, key: KeyEvent) {
        match key.logical_key {
            Key::Character(x) if x.as_str() == "x" => {
                if key.state.is_pressed() {
                    self.distance += 1.0;
                };
            }
            _ => {}
        }
    }

    pub fn pre_render(&mut self) {
        let particle_type = if self.cursor_right_down {
            ParticleType::Empty
        } else {
            self.current_particle_type
        };
        self.fragment_constants = FragmentConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            cursor_down: (self.cursor_down || self.cursor_right_down).into(),
            cursor: self.cursor.into(),
            prev_cursor: self.prev_cursor.into(),
            current_particle_type: particle_type as u32,
            brush_size_sq: self.brush_size * self.brush_size / (self.zoom * self.zoom),
            offset: self.offset,
            zoom: self.zoom,
            debug: self.debug.into(),
        };
        self.prev_cursor = self.cursor;
    }

    pub fn pre_update(&mut self) {
        self.compute_constants = ComputeConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            offset: self.offset,
            zoom: self.zoom,
        };
    }

    pub fn post_update(&mut self) {
        self.offset = 1 - self.offset;
    }

    pub fn fragment_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.fragment_constants)
    }

    pub fn compute_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.compute_constants)
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
        ui.add(egui::Label::new(" Simulation Speed").selectable(false));
        ui.add(
            egui::Slider::new(&mut self.speed, 0.0..=1.99)
                .custom_formatter(|x, _| format!("{:.2}", normalize_speed_up(x as f32)))
                .custom_parser(|x| x.parse().map(|x: f32| normalize_speed_down(x) as f64).ok()),
        );
        ui.add(egui::Label::new("           Zoom").selectable(false));
        ui.add(
            egui::Slider::new(&mut self.zoom, 1.0..=100.0)
                .logarithmic(true)
                .max_decimals(2),
        );
        ui.checkbox(&mut self.debug, "Debug");
    }

    pub fn buffers(&self) -> BufferData {
        BufferData {
            bind_group_buffers: vec![BindGroupBufferType::SSBO(SSBO {
                data: bytemuck::cast_slice(&self.grid.buffer[..]),
                read_only: false,
            })],
        }
    }

    pub fn iterations(&mut self) -> u32 {
        let speed = normalize_speed_up(self.speed);
        let t = self.last_frame.elapsed().as_secs_f32() * 100.0;
        self.last_frame = Instant::now();
        self.distance += speed * t;
        if self.distance >= 1.0 {
            let iterations = self.distance as u32;
            self.distance = self.distance.fract();
            iterations
        } else {
            0
        }
    }
}

fn normalize_speed_down(x: f32) -> f32 {
    (x / 25.0).sqrt()
}

fn normalize_speed_up(x: f32) -> f32 {
    x * x * 25.0
}
