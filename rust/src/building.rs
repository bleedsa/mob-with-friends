use rand::Rng;
use std::{ops::Range, mem};

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
    Uniform,
    Chaos,
});

static FLOORSCHEME_DIMENSION_RANGES: &[(Range<f32>, Range<f32>, Range<f32>)] = &[
    (05.0..30.0, 05.0..30.0, 05.0..10.0),
    (01.0..40.0, 01.0..40.0, 01.0..40.0),
];

impl FloorScheme {
    #[inline(always)]
    pub fn rand() -> Self {
        let mut rng = rand::rng();
        let u = rng.next_u32() % FLOORSCHEME_N;
        unsafe { mem::transmute(u) }
    }

    #[inline(always)]
    pub fn dimension_ranges(&self) -> (Range<f32>, Range<f32>, Range<f32>) {
        FLOORSCHEME_DIMENSION_RANGES[*self as u32 as usize].clone()
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
        let floor_scheme = FloorScheme::rand();
        Self {
            floor_scheme,
            size: Vec3::rand(floor_scheme.dimension_ranges()),
            outer_material: Material::rand_type(MaterialType::BuildingOuter),
            floor_material: Material::rand_type(MaterialType::BuildingFloor),
        }
    }
}
