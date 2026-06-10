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
    #[inline(always)]
    pub fn scene(&self) -> String {
        "res://scenes/constructions/".to_owned() + self.path_part() + ".tscn"
    }

    #[inline(always)]
    pub fn load(&self) -> Gd<Node3D> {
        let path = self.scene();
        let scene = load::<PackedScene>(&path);
        let node = (*scene).instantiate().unwrap();
        node.cast()
    }
}

/*
#[derive(GodotClass)]
#[class(base=Node)]
pub struct GConstruction {
    material: Option<Material>,
    size: Option<Vec3>,
    base: Base<Node>,
}

#[godot_api]
impl INode for GConstruction {
    fn init(base: 
}
*/
