use specs::{Component, VecStorage};

use crate::item::ItemKind;

pub struct Pickable {
    pub kind: ItemKind
}

impl Component for Pickable {
    type Storage = VecStorage<Self>;
}
