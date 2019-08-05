use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
