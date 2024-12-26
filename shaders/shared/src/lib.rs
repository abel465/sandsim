#![cfg_attr(target_arch = "spirv", no_std)]

pub use noise::simplex::noise as rand;

#[cfg(not(target_arch = "spirv"))]
pub mod grid;
pub mod gridref;
pub mod noise;
pub mod particle;
pub mod push_constants;

pub const UI_MENU_HEIGHT: u32 = 22;
pub const UI_SIDEBAR_WIDTH: u32 = 164;
