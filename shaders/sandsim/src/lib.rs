#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sandsim::ShaderConstants;
use shared::gridref::*;
use shared::particle::*;
use shared::*;
use spirv_std::glam::*;
use spirv_std::spirv;

const EMPTY: u32 = 0;
const SAND: u32 = 1;
const WATER: u32 = 2;

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

    if constants.cursor_down.into() {
        let cursor: Vec2 = constants.cursor.into();
        if cursor.distance_squared(frag_coord.xy()) < 1000.0 {
            let tone = rand(
                frag_coord.xy() / vec2(constants.size.width as f32, constants.size.height as f32)
                    * (constants.time % 1.0),
            );
            grid.set(
                frag_coord.x as usize,
                frag_coord.y as usize,
                Particle::sand_from_tone(tone),
            );
        }
    }

    let x = frag_coord.x as usize;
    let y = frag_coord.y as usize;
    let val = grid.get(x, y);
    let col = val.color();
    match val.behaviour {
        EMPTY => {}
        SAND => {
            let s = grid.get(x, y + 1);
            if s.behaviour == EMPTY {
                grid.swap(x, y, x, y + 1)
            }
        }
        WATER => {}
        _ => panic!(),
    }

    *output = col.powf(2.2).extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
