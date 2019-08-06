use specs::{Component, NullStorage};

#[derive(Default)]
pub struct AIControlled;

impl Component for AIControlled {
    type Storage = NullStorage<Self>;
}
