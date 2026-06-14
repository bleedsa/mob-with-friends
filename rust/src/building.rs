use rand::Rng;
use std::{mem, ops::Range};
use godot::prelude::*;

use crate::{material::pre::*, pre::*, map::MapMap, construction::Construction};

pub mod pre {
    pub use super::{BuildingAttrs, FloorScheme, Building};
}

macro_rules! mk_floor_scheme {
    ($enum:ident, $n:ident, $r:ident => {
        $($i:ident => $R:expr),* $(,)*
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(u32)]
        pub enum $enum {
            $($i),*
        }

        pub static $n: u32 = ${count($i)};

        pub static $r: &[(Range<f32>, Range<f32>, Range<f32>)] = &[
            $($R),*
        ];
    };
}

mk_floor_scheme!(FloorScheme, FLOORSCHEME_N, FLOORSCHEME_DIMENSION_RANGES => {
    Uniform => (05.0..30.0, 05.0..30.0, 05.0..10.0),
    Chaos => (01.0..40.0, 01.0..40.0, 01.0..40.0),
});

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
    pub floors: u8,
    pub floor_scheme: FloorScheme,
    pub floor_size: Vec3, 
    pub outer_material: Material,
    pub floor_material: Material,
}

impl BuildingAttrs {
    pub fn rand() -> Self {
        let floor_scheme = FloorScheme::rand();
        Self {
            floors: rand::random_range(1..100),
            floor_scheme,
            floor_size: Vec3::rand(floor_scheme.dimension_ranges()),
            outer_material: Material::rand_type(MaterialType::BuildingOuter),
            floor_material: Material::rand_type(MaterialType::BuildingFloor),
        }
    }
}

impl Default for BuildingAttrs {
    fn default() -> Self {
        Self {
            floors: 0,
            floor_scheme: FloorScheme::Uniform,
            floor_size: Vec3::zero(),
            outer_material: Material::default(),
            floor_material: Material::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuildingWall {
    material: Material,
    position: Vec3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuildingFloor {
    pub attrs: *const BuildingAttrs,
    pub constructions: MapMap<Construction>,
}

impl BuildingFloor {
    /** make a vertical wall */
    #[inline(always)]
    pub fn mk_v_constructions(&mut self, len: usize, mut pos: Vec3, w: Construction) {
        for i in 0..len {
            self.constructions.add(pos, w);
            pos.2 += 1.0;
        }
    }

    /** push walls in the `FloorScheme::Uniform` style */
    #[inline(always)]
    fn push_uniform_walls(&mut self) {
        /* deref the attrs for later use */
        let attrs = unsafe { *self.attrs };
        /* make a wall with the correct outer material */
        let w = Construction::Wall(attrs.outer_material, Vec3::new(1.0, 1.0, 1.0));

        let z = attrs.floor_size;
        self.mk_v_constructions(z.z() as usize, Vec3::new(0.0, 0.0, 0.0), w);
        self.mk_v_constructions(z.z() as usize, Vec3::new(z.x(), 0.0, 0.0), w);
    }

    pub fn rand(attrs: &mut BuildingAttrs) -> Self {
        let mut this = Self {
            attrs: &raw const *attrs,
            constructions: MapMap::new(),
        };

        /* TODO: impl other floor schemes */
        attrs.floor_scheme = FloorScheme::Uniform;

        match attrs.floor_scheme {
            FloorScheme::Uniform => this.push_uniform_walls(),
            _ => todo!(), 
        };

        this
    }
}

#[test]
fn mk_uniform_walls() {
    let mut a = BuildingAttrs::rand();
    a.floor_size = Vec3::new(2.0, 1.0, 4.0);
    a.floor_scheme = FloorScheme::Uniform;

    let mut f = BuildingFloor::rand(&mut a);

    let w_pos = f.constructions.iter()
        .filter(|(_, (_, x))| x.is_wall())
        .map(|(p, _)| *p);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Building {
    pub attrs: BuildingAttrs,
    pub floors: Vec<BuildingFloor>,
}

impl Building {
    #[inline(always)]
    pub fn rand() -> Self {
        let mut this = Self {
            attrs: BuildingAttrs::rand(),
            floors: Vec::new(),
        };

        for _ in 0..this.attrs.floors {
            this.floors.push(BuildingFloor::rand(&mut this.attrs));
        }

        this
    }
}

#[test]
fn generate_random_floors() {
    let b = Building::rand();
    assert_eq!(b.floors.len(), b.attrs.floors.into());
}
