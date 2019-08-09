use specs::{Component, VecStorage};

use crate::item::ItemKind;

pub struct Usable {
    pub kind: ItemKind
}

impl Component for Usable {
    type Storage = VecStorage<Self>;
}
