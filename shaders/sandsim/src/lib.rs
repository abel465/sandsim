#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sandsim::ShaderConstants;
use seq_macro::seq;
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
        let bot_left = grid.get(x, y + 1).behaviour;
        let bot_right = grid.get(x + 1, y + 1).behaviour;

        let mut swap = |p0: [usize; 2], p1: [usize; 2]| {
            grid.swap(x + p0[0], y + p0[1], x + p1[0], y + p1[1]);
        };
        let corner_values = [[top_left, top_right], [bot_left, bot_right]];

        falling_symmetric(&mut swap, corner_values, SAND);
        falling_symmetric(&mut swap, corner_values, WATER);

        seq!(N in 0..=1 {
            let (corner_values, corners) = if N == 0 {
                (
                    corner_values,
                    [TOP_LEFT, TOP_RIGHT, BOT_LEFT, BOT_RIGHT],
                )
            } else {
                (
                    [[top_right, top_left], [bot_right, bot_left]],
                    [TOP_RIGHT, TOP_LEFT, BOT_RIGHT, BOT_LEFT],
                )
            };
            falling_asymmetric(&mut swap, corners, corner_values, SAND);
            falling_asymmetric(&mut swap, corners, corner_values, WATER);
            fluid(&mut swap, corners, corner_values, WATER);
            sand_water_air(&mut swap, corners, corner_values);
        });
    }

    *output = grid.get(x, y).color().powf(2.2).extend(1.0);
}

fn falling_symmetric<F: FnMut([usize; 2], [usize; 2])>(
    swap: &mut F,
    corner_values: [[u32; 2]; 2],
    p: u32,
) {
    if matches!(corner_values, [[a, b], [EMPTY, EMPTY]] if a == p && b == p) {
        swap(TOP_LEFT, BOT_LEFT);
        swap(TOP_RIGHT, BOT_RIGHT);
    }
}

fn falling_asymmetric<F: FnMut([usize; 2], [usize; 2])>(
    swap: &mut F,
    corners: [[usize; 2]; 4],
    corner_values: [[u32; 2]; 2],
    p: u32,
) {
    let [top_left, _, bot_left, _] = corners;
    match corner_values {
        [[a, EMPTY], [EMPTY, EMPTY]] if a == p => swap(top_left, bot_left),
        [[a, EMPTY], [EMPTY, b]] if a == p && b == p => swap(top_left, bot_left),
        [[a, b], [EMPTY, c]] if a == p && b == p && c == p => swap(top_left, bot_left),
        _ => {}
    }
}

fn fluid<F: FnMut([usize; 2], [usize; 2])>(
    swap: &mut F,
    corners: [[usize; 2]; 4],
    corner_values: [[u32; 2]; 2],
    p: u32,
) {
    let [top_left, top_right, bot_left, bot_right] = corners;
    match corner_values {
        [[EMPTY, EMPTY], [a, EMPTY]] if a == p => swap(bot_left, bot_right),
        // [[a, EMPTY], [b, EMPTY]] if a == p && b == p => swap(top_left, bot_right),
        [[a, EMPTY], [b, c]] if a == p && b == p && c == p => swap(top_left, top_right),
        _ => {}
    }
}

fn sand_water_air<F: FnMut([usize; 2], [usize; 2])>(
    swap: &mut F,
    corners: [[usize; 2]; 4],
    corner_values: [[u32; 2]; 2],
) {
    let [top_left, top_right, bot_left, bot_right] = corners;
    match corner_values {
        [[WATER, EMPTY], [SAND, SAND]] => swap(top_left, top_right),
        [[WATER, EMPTY], [SAND, EMPTY]] => swap(top_left, bot_right),
        [[SAND, EMPTY], [WATER, EMPTY]] => {
            swap(bot_left, bot_right);
            swap(top_left, bot_left);
        }
        [[SAND, SAND], [WATER, EMPTY]] => swap(bot_left, bot_right),
        [[EMPTY, SAND], [WATER, EMPTY]] => swap(top_right, bot_right),
        [[EMPTY, WATER], [SAND, EMPTY]] => swap(top_right, bot_right),
        [[WATER, SAND], [EMPTY, EMPTY]] => {
            swap(top_left, bot_left);
            swap(top_right, bot_right);
        }
        [[WATER, WATER], [SAND, EMPTY]] => swap(top_right, bot_right),
        [[WATER, SAND], [WATER, EMPTY]] => {
            swap(bot_left, bot_right);
            swap(top_left, bot_left);
        }
        [[WATER, EMPTY], [WATER, SAND]] => swap(top_left, top_right),
        [[SAND, WATER], [SAND, WATER]] => swap(top_right, bot_right),
        _ => {}
    }
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
