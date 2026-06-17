use godot::{
    classes::{INode, Node},
    meta::AsArg,
    prelude::*,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::{building::pre::*, construction::Construction, item::Item, pre::*};

pub type MapId = u64;

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

    #[inline(always)]
    pub fn len(&self) -> u64 {
        self.0.len() as u64
    }

    /** add an item, doing nothing if there is already something at that position */
    pub fn add_no_matching_pos(&mut self, p: Vec3, x: T) -> MapId {
        /* check if there's already something in this map at position p */
        let mut it = self.0.iter_mut();
        while let Some((i, (q, _))) = it.next() {
            if &p == q {
                return *i;
            }
        }

        self.add(p, x)
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
    T: Clone + Debug + PartialEq,
{
    #[inline(always)]
    fn index_mut(&mut self, index: MapId) -> &mut Self::Output {
        self.0.get_mut(&index).expect(&format!("no key {index} found"))
    }
}

#[test]
fn add_to_MapMap() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    let mut m = MapMap::<u8>::new();
    let id = m.add(v, 100);
    let (_, x) = m[id];

    assert_eq!(x, 100);
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
    /** on new construction not tied to a building */
    #[signal]
    fn on_new_construction(id: MapId);

    /** when a new construction is made on a floor */
    #[signal]
    pub fn on_new_floor_construction(building: MapId, floor: u64, id: MapId);

    /** when a new building is constructed */
    #[signal]
    pub fn on_new_building(building: MapId);

    /** when a new floor is constructed in a building */
    #[signal]
    pub fn on_new_floor(building: MapId, floor: u64);

    #[func]
    fn load_construction(&self, id: MapId) -> Gd<Node3D> {
        let (p, c) = self.constructions[id];
        let mut node = c.load();

        (*node).set_position(p.into());

        node
    }

    #[func]
    fn generate(&mut self) {
        let buildings_num = 100usize;

        for i in 0..buildings_num {
            let (x, y) = idx_to_2d(i, buildings_num / 2, buildings_num / 2);
            let b = Building::rand(i as MapId, &raw mut *self);
            let id = self.buildings.add(Vec3::new(x as f32, 0.0, y as f32), b);
            self.signals().on_new_building().emit(id);
            self.buildings[id].1.generate();
        }
    }

    #[func]
    fn buildings(&self) -> u64 {
        self.buildings.len()
    }

    #[func]
    fn floors(&self, building: MapId) -> u8 {
        self.buildings[building].1.attrs.floors
    }

    #[func]
    fn building_pos(&self, id: MapId) -> Vector3 {
        self.buildings[id].0.into()
    }

    #[func]
    fn building_ids(&self) -> Array<i64> {
        let mut r = Array::new();
        for id in self.buildings.0.keys() {
            r.push(*id as i64);
        }
        r
    }

    #[func]
    fn construction_ids(&self, b: MapId, f: u64) -> Array<i64> {
        let mut r = Array::new();
        for id in self.buildings[b].1.floors[f as usize].constructions.0.keys() {
            r.push(*id as i64);
        }
        r
    }

    #[func]
    fn floor_y(&self, b: MapId, f: u64) -> f32 {
        let mut z = 0.;
        let mut i = 0;

        while i < f {
            z += self.buildings[b].1.floors[f as usize].height;
            i += 1;
        }

        z
    }

    #[func]
    fn load_floor_construction(
        &self,
        building: MapId,
        floor: u64,
        construction: MapId,
    ) -> Gd<Node3D> {
        let b = &self.buildings[building].1;
        let f = &b.floors[floor as usize];
        let (p, c) = &f.constructions[construction];
        let mut node = c.load();
        (*node).set_position(p.into());
        node
    }

    #[func]
    fn load_floor_constructions(&self, building: MapId, floor: u64) -> Array<Gd<Node3D>> {
        let f = &self.buildings[building].1.floors[floor as usize];
        let mut r = Array::new();

        for (i, (p, c)) in f.constructions.iter() {
            let mut node = c.load();
            (*node).set_position(p.into());
            r.insert(*i as usize, &node);
        }

        r
    }
}
