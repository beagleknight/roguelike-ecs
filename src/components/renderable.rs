use specs::VecStorage;
use tcod::Color;
use specs::Component;

pub enum Arrangement {
    Foreground,
    Background
}

pub struct Renderable {
    pub color: Color,
    pub character: Option<char>,
    pub arrangement: Arrangement,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}