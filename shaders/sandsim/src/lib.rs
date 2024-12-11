#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sandsim::ShaderConstants;
use shared::gridref::*;
use shared::particle::*;
use shared::*;
use spirv_std::glam::*;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const EMPTY: u32 = 0;
const SAND: u32 = 1;
const WATER: u32 = 2;

const TOP_LEFT: [usize; 2] = [0, 0];
const TOP_RIGHT: [usize; 2] = [1, 0];
const BOT_LEFT: [usize; 2] = [0, 1];
const BOT_RIGHT: [usize; 2] = [1, 1];

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] grid_buffer: &mut [Particle],
    output: &mut Vec4,
) {
    let mut grid = GridRefMut::new(
        constants.size.width as usize,
        constants.size.height as usize,
        grid_buffer,
    );

    let pos = frag_coord.xy();
    handle_cursor_down(constants, pos, &mut grid);

    let x = pos.x as usize;
    let y = pos.y as usize;
    let offset = constants.offset as usize;
    if x % 2 == offset && y % 2 == offset {
        let top_left = grid.get(x, y).behaviour;
        let top_right = grid.get(x + 1, y).behaviour;
        let bottom_left = grid.get(x, y + 1).behaviour;
        let bottom_right = grid.get(x + 1, y + 1).behaviour;
        let mut swap = |p0: [usize; 2], p1: [usize; 2]| {
            grid.swap(x + p0[0], y + p0[1], x + p1[0], y + p1[1]);
        };
        match [[top_left, top_right], [bottom_left, bottom_right]] {
            [[SAND, EMPTY], [EMPTY, EMPTY]] => swap(TOP_LEFT, BOT_LEFT),
            [[EMPTY, SAND], [EMPTY, EMPTY]] => swap(TOP_RIGHT, BOT_RIGHT),
            [[SAND, SAND], [EMPTY, EMPTY]] => {
                swap(TOP_LEFT, BOT_LEFT);
                swap(TOP_RIGHT, BOT_RIGHT);
            }
            [[SAND, SAND], [EMPTY, SAND]] => swap(TOP_LEFT, BOT_LEFT),
            [[SAND, SAND], [SAND, EMPTY]] => swap(TOP_RIGHT, BOT_RIGHT),
            [[EMPTY, SAND], [SAND, EMPTY]] => swap(TOP_RIGHT, BOT_RIGHT),
            [[SAND, EMPTY], [EMPTY, SAND]] => swap(TOP_LEFT, BOT_LEFT),
            [[EMPTY, SAND], [EMPTY, SAND]] => swap(TOP_RIGHT, BOT_LEFT),
            [[SAND, EMPTY], [SAND, EMPTY]] => swap(TOP_LEFT, BOT_RIGHT),
            _ => {}
        };
    }

    *output = grid.get(x, y).color().powf(2.2).extend(1.0);
}

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

fn handle_cursor_down(constants: &ShaderConstants, pos: Vec2, grid: &mut GridRefMut<Particle>) {
    if constants.cursor_down.into() {
        let prev_cursor: Vec2 = constants.prev_cursor.into();
        let cursor: Vec2 = constants.cursor.into();
        if distance_sq_to_line_segment(pos, prev_cursor, cursor) < constants.brush_size_sq {
            let tone = rand(pos / constants.size.as_vec2() * (constants.time % 1.0));
            let particle_type = ParticleType::from_value(constants.current_particle_type);
            let particle = Particle::new_from_tone(particle_type, tone);
            grid.set(pos.x as usize, pos.y as usize, particle);
        }
    }
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
