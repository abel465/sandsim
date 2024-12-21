pub mod simplex;

use spirv_std::glam::*;

fn mod289_vec3(x: Vec3) -> Vec3 {
    x - (x * (1.0 / 289.0)).floor() * 289.0
}

fn mod289_vec2(x: Vec2) -> Vec2 {
    x - (x * (1.0 / 289.0)).floor() * 289.0
}

fn permute(x: Vec3) -> Vec3 {
    mod289_vec3(((x * 34.0) + 10.0) * x)
}
