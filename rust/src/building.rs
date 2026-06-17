use godot::prelude::*;
use rand::Rng;
use std::{hint::likely, mem, ops::Range};

use crate::{
    construction::Construction,
    map::{Map, MapId, MapMap},
    material::pre::*,
    pre::*,
};

pub mod pre {
    pub use super::{Building, BuildingAttrs, FloorScheme};
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
    pub id: MapId,
    pub floors: u8,
    pub floor_scheme: FloorScheme,
    pub floor_size: Vec3,
    pub outer_material: Material,
    pub floor_material: Material,
    pub map: *mut Map,
}

impl BuildingAttrs {
    pub fn rand(id: MapId, map: *mut Map) -> Self {
        let floor_scheme = FloorScheme::rand();
        Self {
            id,
            floors: rand::random_range(1..100),
            floor_scheme,
            floor_size: Vec3::rand(floor_scheme.dimension_ranges()),
            outer_material: Material::rand_type(MaterialType::BuildingOuter),
            floor_material: Material::rand_type(MaterialType::BuildingFloor),
            map,
        }
    }
}

impl Default for BuildingAttrs {
    fn default() -> Self {
        Self {
            id: 0,
            floors: 0,
            floor_scheme: FloorScheme::Uniform,
            floor_size: Vec3::zero(),
            outer_material: Material::default(),
            floor_material: Material::default(),
            map: std::ptr::null_mut(),
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
    pub constructions: MapMap<Construction>,
    pub height: f32,
    pub attrs: *mut BuildingAttrs,
}

impl BuildingFloor {
    /** push walls in the `FloorScheme::Uniform` style */
    #[inline(always)]
    fn push_uniform_walls(&mut self) {
        /* deref the attrs for later use */
        let attrs = unsafe { *self.attrs };
        /* make a wall with the correct outer material */
        let w = Construction::Wall(attrs.outer_material, Vec3::new(1.0, 1.0, 1.0));

        let z = attrs.floor_size;

        /* lol */
        let C = &raw mut self.constructions;

        /* make a line of vertical walls */
        let mk_v_w = |len: f32, mut pos: Vec3| {
            for _ in 0..len as usize {
                unsafe { (&mut *C).add_no_matching_pos(pos, w) };
                pos.2 += 1.;
            }
        };

        /* make a line of horizontal walls */
        let mk_h_w = |len: f32, mut pos: Vec3| {
            for _ in 0..len as usize {
                unsafe { (&mut *C).add_no_matching_pos(pos, w) };
                pos.0 += 1.;
            }
        };

        /* verts */
        mk_v_w(z.z(), Vec3::new(0., 0., 0.));
        mk_v_w(z.z(), Vec3::new(z.x(), 0., 0.));

        /* horizs */
        mk_h_w(z.x(), Vec3::new(0., 0., 0.));
        mk_h_w(z.x(), Vec3::new(0., 0., z.z()));
    }

    pub fn rand(attrs: *mut BuildingAttrs) -> Self {
        let mut this = Self {
            attrs,
            height: unsafe { (*attrs).floor_size.y() },
            constructions: MapMap::new(),
        };

        unsafe {
            /* TODO: impl other floor schemes */
            (*attrs).floor_scheme = FloorScheme::Uniform;

            match (*attrs).floor_scheme {
                FloorScheme::Uniform => this.push_uniform_walls(),
                _ => todo!(),
            };
        }

        this
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Building {
    pub attrs: BuildingAttrs,
    pub floors: Vec<BuildingFloor>,
}

impl Building {
   #[inline(always)]
    pub fn map<F>(&self, f: F)
    where
        F: Fn(&mut Map) -> (),
    {
        let m = self.attrs.map;
        if likely(m != std::ptr::null_mut()) {
            unsafe { f(&mut *m) };
        }
    }

    pub fn rand(id: MapId, map: *mut Map) -> Self {
        Self::rand_with_attrs(BuildingAttrs::rand(id, map))
    }
 
    #[inline(always)]
    pub fn rand_with_attrs(attrs: BuildingAttrs) -> Self {
        Self {
            attrs,
            floors: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        for f in 0..self.attrs.floors as u64 {
            self.floors.push(BuildingFloor::rand(&raw mut self.attrs));
        }
    }
}

#[test]
fn generate_uniform_floors() {
    let b = Building::rand_with_attrs(BuildingAttrs {
        floor_scheme: FloorScheme::Uniform,
        floor_size: Vec3::new(3., 1., 4.),
        ..BuildingAttrs::default()
    });
    assert_eq!(b.floors.len(), b.attrs.floors.into());

    for f in b.floors.iter() {
        /* wall positions */
        let mut w_pos: Vec<_> = f
            .constructions
            .iter()
            .filter(|(_, (_, x))| x.is_wall())
            .map(|(i, (p, _))| (*i, *p))
            .collect();

        /* sort by id */
        w_pos.sort_by(|(q, _), (r, _)| q.cmp(r));

        /* map out ids */
        let w_pos: Vec<_> = w_pos.into_iter().map(|(_, x)| x).collect();

        assert_eq!(
            &w_pos,
            &[
                /* verts */
                Vec3::zero(),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 2.),
                Vec3::new(0., 0., 3.),
                Vec3::new(3., 0., 0.),
                Vec3::new(3., 0., 1.),
                Vec3::new(3., 0., 2.),
                Vec3::new(3., 0., 3.),
                /* horzs */
                Vec3::new(1., 0., 0.),
                Vec3::new(2., 0., 0.),
                Vec3::new(0., 0., 4.),
                Vec3::new(1., 0., 4.),
                Vec3::new(2., 0., 4.),
            ]
        );
    }
}
