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

    let pos = frag_coord.xy();
    handle_cursor_down(constants, pos, &mut grid);

    let x = pos.x as usize;
    let y = pos.y as usize;
    let val = grid.get(x, y);
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

    *output = val.color().powf(2.2).extend(1.0);
}

fn handle_cursor_down(constants: &ShaderConstants, pos: Vec2, grid: &mut GridRefMut<Particle>) {
    if constants.cursor_down.into() {
        let cursor: Vec2 = constants.cursor.into();
        if cursor.distance_squared(pos) < 1000.0 {
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
