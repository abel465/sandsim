#![cfg_attr(target_arch = "spirv", no_std)]

use dfutils::gridref::*;
use push_constants::sandsim::ShaderConstants;
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
        if cursor.distance_squared(frag_coord.xy()) < 100.0 {
            grid.set(
                frag_coord.x as usize,
                frag_coord.y as usize,
                Particle::new_sand2(),
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
