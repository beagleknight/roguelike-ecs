use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Fighter {
    pub base_defense: i32,
    pub base_power: i32,
}

impl Component for Fighter {
    type Storage = VecStorage<Self>;
}
