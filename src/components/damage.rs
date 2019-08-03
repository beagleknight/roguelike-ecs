use specs::{Component, VecStorage};

#[derive(Default)]
pub struct Damage {
    pub base: i32,
}

impl Component for Damage {
    type Storage = VecStorage<Self>;
}
