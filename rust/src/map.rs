use godot::{
    classes::{INode, Node},
    meta::AsArg,
    prelude::*,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    hint::unreachable_unchecked,
    ops::{Index, IndexMut},
};

use crate::{construction::Construction, item::Item, pre::*, building::pre::*};

type MapId = u64;

/** a map (ie key value pairs) of things on a map (ie where gameplay takes place).
 *
 * a `MapMap` contains a map from `MapId`s to a position and item. This lets you
 * position items on the map using unique ids. id handling is done automatically
 * by the provided methods. */
#[derive(Clone, Debug, PartialEq)]
pub struct MapMap<T>(pub HashMap<MapId, (Vec3, T)>, pub MapId)
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
    pub fn add(&mut self, p: Vec3, x: T) -> MapId {
        /* skip any existing ids */
        while let Some(_) = self.0.get(&self.1) {
            if self.1 >= MapId::MAX {
                self.1 = 0;
            }

            self.1 += 1;
        }

        /* insert at the next id */
        self.0.insert(self.1, (p, x));
        self.1
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&MapId, &(Vec3, T))> {
        self.0.iter()
    }
}

impl<T> Index<MapId> for MapMap<T>
where
    T: Clone + Debug + PartialEq,
{
    type Output = (Vec3, T);

    #[inline(always)]
    fn index(&self, index: MapId) -> &Self::Output {
        &self.0[&index]
    }
}

impl<T> IndexMut<MapId> for MapMap<T>
where
    T: Clone + Debug + PartialEq + Default,
{
    #[inline(always)]
    fn index_mut(&mut self, index: MapId) -> &mut Self::Output {
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
#[class(base=Node)]
pub struct Map {
    pub constructions: MapMap<Construction>,
    pub buildings: MapMap<Building>,
    pub items: MapMap<Item>,
    pub base: Base<Node>,
}

impl Map {
    pub fn add_node(&mut self, x: impl AsArg<Option<Gd<Node>>>) {
        self.base_mut().add_child(x)
    }

    pub fn new_construction_(&mut self, p: Vec3, c: Construction) -> MapId {
        let id = self.constructions.add(p, c);
        self.signals().on_new_construction().emit(id);
        id
    }
}

#[godot_api]
impl INode for Map {
    fn init(base: Base<Node>) -> Self {
        Self {
            constructions: MapMap::new(),
            items: MapMap::new(),
            buildings: MapMap::new(),
            base,
        }
    }
}

#[godot_api]
impl Map {
    #[signal]
    fn on_new_construction(id: MapId);

    #[func]
    fn load_construction(&self, id: MapId) -> Gd<Node3D> {
        let (p, c) = self.constructions[id];
        let mut node = c.load();

        (*node).set_position(p.into());

        node
    }

    #[func]
    fn generate(&mut self) {
        let buildings_num = 100;

        for i in 0..buildings_num {
            let b = Building::rand();
            let (x, y) = idx_to_2d(i, buildings_num/2, buildings_num/2);
            self.buildings.add(Vec3::new(x as f32, y as f32, 0.0), b);
        }
    }
}
