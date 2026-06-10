use godot::prelude::*;

use crate::{material::Material, pre::*};

#[macro_export]
macro_rules! mk_constructions {
    (@NilTy $t:ty) => { _ };

    ($n:ident => { $($x:ident($($t:ty),*) => $y:expr),* $(,)* }) => {
        /** a construction on the map */
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum $n {
            $($x($($t),*)),*
        }

        impl $n {
            #[inline(always)]
            pub fn path_part(&self) -> &'static str {
                use $n::*;
                match self {
                    $($x($($crate::mk_constructions!(@NilTy $t)),*) => $y),*
                }
            }
        }
    };
}

mk_constructions!(Construction => {
    Wall(Material, Vec3) => "wall",
    Floor(Material, Vec3) => "floor",
});

impl Construction {
    pub fn load(&self) -> Gd<Node3D> {
        let path = "res://scenes/constructions/".to_owned() + self.path_part();
        let scene = load::<PackedScene>(&path);
        let node = (*scene).instantiate().unwrap();
        node.cast()
    }
}
