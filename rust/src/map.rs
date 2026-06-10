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

use crate::{construction::Construction, item::Item, pre::*};

/** a map (ie key value pairs) of things on a map (ie where gameplay takes place). */
#[derive(Clone, Debug, PartialEq)]
pub struct MapMap<T>(pub HashMap<u64, (Vec3, T)>, pub u64)
where
    T: Clone + Debug + PartialEq;

impl<T> MapMap<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn new() -> Self {
        Self(HashMap::new(), 0)
    }

    /** add an item x at position p */
    #[inline(always)]
    pub fn add(&mut self, p: Vec3, x: T) -> &(Vec3, T) {
        /* skip any existing ids */
        while let Some(_) = self.0.get(&self.1) {
            if self.1 >= u64::MAX {
                self.1 = 0;
            }

            self.1 += 1;
        }

        /* insert at the next id */
        self.0.insert(self.1, (p, x));
        &self.0[&self.1]
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item=(&u64, &(Vec3, T))> {
        self.0.iter()
    }
}

impl<T> Index<u64> for MapMap<T>
where
    T: Clone + Debug + PartialEq,
{
    type Output = (Vec3, T);

    #[inline(always)]
    fn index(&self, index: u64) -> &Self::Output {
        &self.0[&index]
    }
}

impl<T> IndexMut<u64> for MapMap<T>
where
    T: Clone + Debug + PartialEq + Default,
{
    #[inline(always)]
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        let ptr = &raw mut self.0;
        unsafe {
            if let Some(x) = (*ptr).get_mut(&index) {
                return x;
            }

            (*ptr).insert(index, (Vec3::zero(), T::default()));
            if let Some(x) = (*ptr).get_mut(&index) {
                x
            } else {
                unreachable_unchecked()
            }
        }
    }
}

#[test]
fn add_to_MapMap() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    let mut m = MapMap::<u8>::new();
    let (_, x) = m.add(v, 100);

    assert_eq!(*x, 100);
}

#[test]
fn IndexMut_MapMap() {
    let mut m = MapMap::new();
    let (_, x) = &mut m[0];
    *x = 100;

    assert_eq!(*x, 100);
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Map {
    pub constructions: MapMap<Construction>,
    pub items: MapMap<Item>,
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
            constructions: MapMap::new(),
            items: MapMap::new(),
            base,
        }
    }
}

#[godot_api]
impl Map {
    #[signal]
    fn new_construction(id: u64);
}
