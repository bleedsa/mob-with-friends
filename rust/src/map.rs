use godot::{
    meta::AsArg,
    classes::{INode3D, Node3D},
    prelude::*,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    hint::unreachable_unchecked,
    ops::{Index, IndexMut},
};

/** a position map for items, constructions, etc on the map */
use crate::{construction::Construction, item::Item, pre::*};

#[derive(Clone, Debug, PartialEq)]
pub struct PosMap<T>(pub HashMap<Vec3, T>)
where
    T: Clone + Debug + PartialEq;

impl<T> PosMap<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline(always)]
    pub fn add(&mut self, k: Vec3, x: T) -> &T {
        self.0.insert(k, x);
        &self.0[&k]
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item=(&Vec3, &T)> {
        self.0.iter()
    }
}

impl<T> Index<Vec3> for PosMap<T>
where
    T: Clone + Debug + PartialEq,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Vec3) -> &Self::Output {
        &self.0[&index]
    }
}

impl<T> IndexMut<Vec3> for PosMap<T>
where
    T: Clone + Debug + PartialEq + Default,
{
    #[inline(always)]
    fn index_mut(&mut self, index: Vec3) -> &mut Self::Output {
        let ptr = &raw mut self.0;
        unsafe {
            if let Some(x) = (*ptr).get_mut(&index) {
                return x;
            }

            (*ptr).insert(index, T::default());
            if let Some(x) = (*ptr).get_mut(&index) {
                x
            } else {
                unreachable_unchecked()
            }
        }
    }
}

#[test]
fn add_to_PosMap() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    let mut m = PosMap::<u8>::new();
    m.add(v, 100);

    assert_eq!(m[v], 100);
}

#[test]
fn IndexMut_PosMap() {
    let v = Vec3::new(0.0, 1.0, 2.0);
    let mut m = PosMap::new();
    m[v] = 100;

    assert_eq!(m[v], 100);
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Map {
    pub constructions: PosMap<Construction>,
    pub items: PosMap<Item>,
    pub base: Base<Node3D>,
}

impl Map {
    pub fn add_node(&mut self, x: impl AsArg<Option<Gd<Node>>>) {
        self.base_mut().add_child(x)
    }
}

#[godot_api]
impl INode3D for Map {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            constructions: PosMap::new(),
            items: PosMap::new(),
            base,
        }
    }

    fn process(&mut self, _delta: f32) {
        let this = &raw mut *self; /* whatever */

        for (_, v) in self.constructions.iter() {
            let node = v.load();
            unsafe { &mut *this }.add_node(&node);
        }
    }
}
