#![cfg_attr(target_arch = "spirv", no_std)]

use spirv_std::glam::*;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

#[cfg(not(target_arch = "spirv"))]
pub mod grid;
pub mod gridref;
pub mod particle;
pub mod push_constants;

pub fn fullscreen_vs(vert_id: i32, out_pos: &mut Vec4) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}

pub fn rand(co: Vec2) -> f32 {
    let a = 12.9898;
    let b = 78.233;
    let c = 43758.5453;
    let dt = co.dot(vec2(a, b));
    let sn = dt % 3.14;
    (sn.sin() * c).fract()
}
