use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Pickable;

impl Component for Pickable {
    type Storage = NullStorage<Self>;
}
