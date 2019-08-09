use specs::{Component, VecStorage};

use crate::item::SlotKind;

#[derive(Clone, PartialEq)]
pub struct Equipable {
    pub max_hp_bonus: i32,
    pub power_bonus: i32,
    pub defense_bonus: i32,
    pub slot: SlotKind,
}

impl Component for Equipable {
    type Storage = VecStorage<Self>;
}
