use rand::Rng;
use std::mem;

use crate::{material::pre::*, pre::*};

pub mod pre {
    pub use super::{BuildingAttrs, FloorScheme};
}

macro_rules! mk_floor_scheme {
    ($enum:ident, $n:ident => { $($i:ident),* $(,)* }) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(u32)]
        pub enum $enum {
            $($i),*
        }

        pub static $n: u32 = ${count($i)};
    };
}

mk_floor_scheme!(FloorScheme, FLOORSCHEME_N => {
    UniformSquare,
    Chaos,
});

impl FloorScheme {
    pub fn rand() -> Self {
        let mut rng = rand::rng();
        let u = rng.next_u32() % FLOORSCHEME_N;
        unsafe { mem::transmute(u) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuildingAttrs {
    pub floor_scheme: FloorScheme,
    pub size: Vec3,
    pub outer_material: Material,
    pub floor_material: Material,
}

impl BuildingAttrs {
    pub fn rand() -> Self {
        Self {
            floor_scheme: FloorScheme::rand(),
            size: Vec3::rand(),
            outer_material: Material::rand_type(MaterialType::BuildingOuter),
            floor_material: Material::rand_type(MaterialType::BuildingFloor),
        }
    }
}
