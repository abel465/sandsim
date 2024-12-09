#![cfg_attr(target_arch = "spirv", no_std)]

use spirv_std::glam::*;

pub mod particle;
pub mod push_constants;

pub fn fullscreen_vs(vert_id: i32, out_pos: &mut Vec4) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}
