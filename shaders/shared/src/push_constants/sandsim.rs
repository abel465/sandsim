use super::{Bool, Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct FragmentConstants {
    pub size: Size,
    pub cursor: Vec2,
    pub prev_cursor: Vec2,
    pub time: f32,
    pub cursor_down: Bool,
    pub current_particle_type: u32,
    pub brush_size_sq: f32,
    pub zoom: f32,
}

impl FragmentConstants {
    pub fn mem_size() -> usize {
        core::mem::size_of::<Self>()
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ComputeConstants {
    pub size: Size,
    pub time: f32,
    pub offset: u32,
    pub zoom: f32,
}

impl ComputeConstants {
    pub fn mem_size() -> usize {
        core::mem::size_of::<Self>()
    }
}
