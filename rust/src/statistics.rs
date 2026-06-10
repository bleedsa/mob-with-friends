use godot::{
    classes::{INode, Node},
    prelude::*,
};

use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub mod pre {
    pub use super::Statistics;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statistics {
    pub materials_rejected: usize,
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

pub fn new_material_rejected() -> usize {
    println!("material rejected");
    let mut lock = STATISTICS.write().expect("poisoned RwLock");
    (*lock).materials_rejected += 1;
    (*lock).materials_rejected
}

pub fn materials_rejected() -> usize {
    let lock = STATISTICS.read().expect("poisoned RwLock");
    (*lock).materials_rejected
}

#[test]
fn reject_a_material() {
    new_material_rejected();
    assert_eq!(materials_rejected(), 1);
}

mod export {
    use super::*;

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

    #[godot_api]
    impl Stats {
        #[func]
        fn reset() {
            let mut G = STATISTICS.write().expect("poisoned RwLock");
            (*G) = Statistics::new();
        }

        #[func]
        fn materials_rejected() -> u32 {
            let G = STATISTICS.read().expect("poisoned RwLock");
            (*G).materials_rejected as u32
        }
    }
}
