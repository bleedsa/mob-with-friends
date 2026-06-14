#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]

use flt::HashFlt;
use godot::prelude::*;
use std::ops::Range;

pub mod building;
pub mod construction;
pub mod flt;
pub mod item;
pub mod map;
pub mod material;
pub mod statistics;

pub mod pre {
    pub use super::{Vec3, idx_to_2d};
}

struct BleedThorn;

#[gdextension]
unsafe impl ExtensionLibrary for BleedThorn {}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Vec3(pub HashFlt, pub HashFlt, pub HashFlt);

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(HashFlt::new(x), HashFlt::new(y), HashFlt::new(z))
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /** generate a random vector with values between the ranges specified in `ranges`. */
    #[inline(always)]
    pub fn rand((x, y, z): (Range<f32>, Range<f32>, Range<f32>)) -> Self {
        Self(HashFlt::rand(x), HashFlt::rand(y), HashFlt::rand(z))
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.0.into()
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.1.into()
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.2.into()
    }
}

impl Into<Vector3> for Vec3 {
    fn into(self) -> Vector3 {
        Vector3 {
            x: self.0.into(),
            y: self.1.into(),
            z: self.2.into(),
        }
    }
}

pub fn idx_to_2d(idx: usize, X: usize, Y: usize) -> (usize, usize) {
    (idx / X, idx % Y)
}
