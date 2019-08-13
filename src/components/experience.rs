use specs::{Component, VecStorage};

pub struct Experience {
    pub level: u32,
    pub points: u32,
    pub next_level_points: u32
}

impl Component for Experience {
    type Storage = VecStorage<Experience>;
}