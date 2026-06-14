use godot::{
    classes::{INode, Node},
    prelude::*,
};

use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub mod pre {
    pub use super::Statistics;
}

/** the statistics object exported to godot */
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    base: Base<Node>,
}

#[godot_api]
impl INode for Stats {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

macro_rules! mk_statistics {
    {
        pub struct $n:ident {
            $($x:ident: $t:ty => $into:ty),* $(,)*
        }
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $n {
            $(pub $x: $t),*
        }

        $(
            pub fn $x() -> $t {
                let lock = STATISTICS
                    .read()
                    .expect(&format!("poisoned RwLock in {}()", stringify!($x)));
                (*lock).$x
            }
        ),*

        #[godot_api]
        impl Stats {
            /*
             * ====================================================================================
             * DUE TO LIMITATIONS WITH GODOT-RS, ALL EXPORTED METHODS TO STATS MUST BE PUT WITHIN
             * THIS MACRO DECLARATION. SORRY LIBERALS.
             * ====================================================================================
             */

            /** reset the statistics */
            #[func]
            fn reset() {
                let mut G = STATISTICS.write().expect("poisoned RwLock");
                (*G) = Statistics::new();
            }

            $(
                #[func]
                pub fn $x() -> $into {
                    let lock = STATISTICS
                        .read()
                        .expect(&format!("poisoned RwLock in Stats::{}()", stringify!($x)));
                    (*lock).$x as $into
                }
            ),*
        }
    };
}

mk_statistics! {
    pub struct Statistics {
        materials_rejected: usize => u32,
    }
}

impl Statistics {
    pub const fn new() -> Self {
        Self {
            materials_rejected: 0,
        }
    }
}

static STATISTICS: RwLock<Statistics> = RwLock::new(Statistics::new());

pub type WriteErr = PoisonError<RwLockWriteGuard<'static, Statistics>>;
pub type ReadErr = PoisonError<RwLockReadGuard<'static, Statistics>>;

macro_rules! W {
    ($stat:ident += $x:expr) => {{
        let mut lock = STATISTICS.write().expect("poisoned RwLock in W!()");
        (*lock).$stat += $x;
        (*lock).$stat
    }};
}

#[inline(always)]
pub fn new_material_rejected() {
    W!(materials_rejected += 1);
}
