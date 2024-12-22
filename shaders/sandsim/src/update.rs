use seq_macro::seq;
use shared::gridref::*;
use shared::particle::*;
use spirv_std::glam::*;

const EMPTY: u32 = 0;
const SAND: u32 = 1;
const WATER: u32 = 2;

const TOP_LEFT: [usize; 2] = [0, 0];
const TOP_RIGHT: [usize; 2] = [1, 0];
const BOT_LEFT: [usize; 2] = [0, 1];
const BOT_RIGHT: [usize; 2] = [1, 1];

pub fn update(pos: UVec2, grid: &mut GridRefMut<Particle>) {
    let x = pos.x as usize;
    let y = pos.y as usize;
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
        [[SAND, WATER], [SAND, EMPTY]] => swap(top_right, bot_right),
        _ => {}
    }
}
