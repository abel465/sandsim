use bytemuck::{Pod, Zeroable};
use spirv_std::glam::*;

fn rgb(x: u32) -> Vec3 {
    uvec3(x >> 16, (x >> 8) & 0xFF, x & 0xFF).as_vec3() / 255.0
}

#[derive(Default, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum ParticleType {
    #[default]
    Empty,
    Sand,
    Water,
}

impl ParticleType {
    pub fn from_value(value: u32) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Sand,
            2 => Self::Water,
            _ => panic!("Invalid value"),
        }
    }

    pub fn color_range(&self) -> (Vec3, Vec3) {
        match self {
            Self::Empty => (rgb(0xE8E6E3), rgb(0xDDDDEE)),
            Self::Sand => (rgb(0xDDC594), rgb(0xC2B47C)),
            Self::Water => (rgb(0x428EF1), rgb(0x24B6FF)),
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Particle {
    pub behaviour: u32,
    pub tone: f32,
}

impl Particle {
    fn color_range(&self) -> (Vec3, Vec3) {
        ParticleType::from_value(self.behaviour).color_range()
    }

    pub fn color(&self) -> Vec3 {
        let (c1, c2) = self.color_range();
        c1.lerp(c2, self.tone)
    }

    pub fn new_from_tone(behaviour: ParticleType, tone: f32) -> Self {
        Self {
            behaviour: behaviour as u32,
            tone,
        }
    }

    pub fn sand_from_tone(tone: f32) -> Self {
        Self::new_from_tone(ParticleType::Sand, tone)
    }

    pub fn empty_from_tone(tone: f32) -> Self {
        Self::new_from_tone(ParticleType::Empty, tone)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Particle {
    pub fn new(behaviour: ParticleType) -> Self {
        Self {
            behaviour: behaviour as u32,
            tone: rand::random(),
        }
    }

    pub fn new_sand() -> Self {
        Self::new(ParticleType::Sand)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Particle {
    fn default() -> Self {
        Self::new(ParticleType::Empty)
    }
}
