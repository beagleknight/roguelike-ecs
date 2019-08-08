use specs::{Component, Entity, VecStorage};

pub struct Inventory {
    pub objects: Vec<Entity>,
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}
