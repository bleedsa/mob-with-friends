use rand::Rng;
use std::mem;

use crate::statistics;

pub mod pre {
    pub use super::{Material, MaterialType};
}

macro_rules! mk_types {
    ($t:ident, $v:ident => { $($x:ident => [$($n:ident),*]),* $(,)* }) => {
        /** what type of material do you want?
         * useful for specifying specific material subtypes for world generation. */
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(usize)]
        pub enum $t {
            $($x),*
        }

        pub static $v: &[&[Material]] = &[$(&[$(Material::$n),*]),*];
    };
}

macro_rules! mk_materials {
    ($m:ident, $n:ident, $id:ident, $name:ident => {
        $($x:ident is $y:expr),* $(,)*
    }) => {
        /** a material that constructions, objects, etc can be made of. */
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(u32)]
        pub enum $m {
            $($x),*
        }

        /** the number of materials there are */
        pub static $n: usize = ${count($x)};
        /** the string id of a material */
        pub static $id: &[&str] = &[$(stringify!($x)),*];
        /** the name of a material */
        pub static $name: &[&str] = &[$($y),*];
    };
}

mk_types!(MaterialType, TYPE_VALIDS => {
    /* metal materials */
    Metals => [Iron, Copper, Gold, Silver, Platinum],
    /* materials that are used in the outside of buildings */
    BuildingOuter => [Brick, GreyStone],
    /* building floor materials */
    BuildingFloor => [Brick, GreyStone, Concrete],
});

impl MaterialType {
    #[inline(always)]
    pub fn check(&self, x: Material) -> bool {
        let valids = TYPE_VALIDS[*self as usize];
        valids.contains(&x)
    }
}

mk_materials!(Material, MATERIAL_N, MATERIAL_IDS, MATERIAL_NAMES => {
    Brick is "brick",
    GreyStone is "grey stone",
    Iron is "iron" ,
    Copper is "copper",
    Gold is "gold",
    Silver is "silver",
    Platinum is "platinum",
    Concrete is "concrete",
});

impl Material {
    fn rand_(mut rng: impl Rng) -> Self {
        let u = rng.next_u32() % MATERIAL_N as u32;
        unsafe { mem::transmute(u) }
    }

    /** make a random material.
     * SEE ALSO: `Self::rand_type()` for random materials with specific properties. */
    pub fn rand() -> Self {
        let mut rng = rand::rng();
        Self::rand_(&mut rng)
    }

    /** make a random material within a validated range */
    pub fn rand_type(t: MaterialType) -> Self {
        let mut rng = rand::rng();
        loop {
            let m = Self::rand_(&mut rng);
            if t.check(m) {
                return m;
            } else {
                statistics::new_material_rejected();
            }
        }
    }
}
