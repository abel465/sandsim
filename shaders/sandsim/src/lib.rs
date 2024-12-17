#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sandsim::*;
use shared::gridref::*;
use shared::particle::*;
use shared::*;
use spirv_std::glam::*;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

mod update;

fn distance_sq_to_line_segment(p: Vec2, v: Vec2, w: Vec2) -> f32 {
    // Return the distance squared between point p and line segment vw
    let l2 = v.distance_squared(w); // i.e. |w-v|^2 -  avoid a sqrt
    if l2 == 0.0 {
        return p.distance_squared(v); // v == w case
    }
    // Consider the line extending the segment, parameterized as v + t (w - v).
    // We find projection of point p onto the line.
    // It falls where t = [(p-v) . (w-v)] / |w-v|^2
    // We clamp t from [0,1] to handle points outside the segment vw.
    let t = 0.0.max(1.0.min((p - v).dot(w - v) / l2));
    let projection = v + t * (w - v); // Projection falls on the segment
    return p.distance_squared(projection);
}

fn handle_cursor_down(constants: &FragmentConstants, pos: Vec2, grid: &mut GridRefMut<Particle>) {
    if constants.cursor_down.into() {
        let height = constants.size.height;
        let prev_cursor = zoom(constants.prev_cursor.into(), height, constants.zoom);
        let cursor = zoom(constants.cursor.into(), height, constants.zoom);
        if distance_sq_to_line_segment(pos, prev_cursor, cursor) < constants.brush_size_sq {
            let tone = rand(pos / constants.size.as_vec2() * (constants.time % 1.0));
            let particle_type = ParticleType::from_value(constants.current_particle_type);
            let particle = Particle::new_from_tone(particle_type, tone);
            grid.set(pos.x as usize, pos.y as usize, particle);
        }
    }
}

fn zoom(p: Vec2, height: u32, zoom: f32) -> Vec2 {
    let height = height as f32;
    p / zoom + Vec2::Y * (height - height / zoom)
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &FragmentConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] grid_buffer: &mut [Particle],
    output: &mut Vec4,
) {
    let mut grid = GridRefMut::new(
        constants.size.width as usize,
        constants.size.height as usize,
        grid_buffer,
    );

    let pos = zoom(frag_coord.xy(), constants.size.height, constants.zoom);
    handle_cursor_down(constants, pos, &mut grid);

    *output = grid
        .get(pos.x as usize, pos.y as usize)
        .color()
        .powf(2.2)
        .extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}

#[spirv(compute(threads(16, 16)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(push_constant)] constants: &ComputeConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] grid_buffer: &mut [Particle],
) {
    let mut grid = GridRefMut::new(
        constants.size.width as usize,
        constants.size.height as usize,
        grid_buffer,
    );
    let pos = gid.xy() * 2 + constants.offset;
    update::update(pos, &mut grid);
}
