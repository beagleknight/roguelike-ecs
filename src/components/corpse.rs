use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Corpse;

impl Component for Corpse {
    type Storage = NullStorage<Self>;
}
