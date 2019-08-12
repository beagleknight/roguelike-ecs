use specs::{VecStorage, Component};

pub struct Stairs {
    pub from_level: u32,
    pub to_level: u32,
}

impl Component for Stairs {
    type Storage = VecStorage<Stairs>;
}