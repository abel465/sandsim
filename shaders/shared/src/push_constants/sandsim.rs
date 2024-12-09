use super::{Bool, Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub cursor: Vec2,
    pub prev_cursor: Vec2,
    pub time: f32,
    pub cursor_down: Bool,
    pub current_particle_type: u32,
    pub brush_size_sq: f32,
}
