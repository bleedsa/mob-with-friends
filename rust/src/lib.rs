#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]

use godot::prelude::*;

pub mod building;
pub mod construction;
pub mod flt;
pub mod item;
pub mod map;
pub mod material;
pub mod statistics;

pub mod pre {
    pub use super::Vec3;
}

use flt::HashFlt;

struct BleedThorn;

#[gdextension]
unsafe impl ExtensionLibrary for BleedThorn {}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Vec3(pub HashFlt, pub HashFlt, pub HashFlt);

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(HashFlt::new(x), HashFlt::new(y), HashFlt::new(z))
    }

    #[inline(always)]
    pub fn rand() -> Self {
        let mut rng = rand::rng();
        Self(HashFlt::rand(&mut rng), HashFlt::rand(&mut rng), HashFlt::rand(&mut rng))
    }

}
