use specs::VecStorage;
use tcod::Color;
use specs::Component;

pub struct Renderable {
    pub color: Color,
    pub character: char,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}