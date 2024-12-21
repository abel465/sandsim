#![cfg_attr(target_arch = "spirv", no_std)]

pub use noise::simplex::noise as rand;

#[cfg(not(target_arch = "spirv"))]
pub mod grid;
pub mod gridref;
pub mod noise;
pub mod particle;
pub mod push_constants;
