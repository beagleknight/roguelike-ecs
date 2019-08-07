use specs::{Component, VecStorage};

use crate::map::TileKind;

pub struct Tile {
    pub explored: bool,
    pub kind: TileKind,
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}
