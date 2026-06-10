use crate::material::Material;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Item {
    TommyGun,
    Coin(Material),
}
