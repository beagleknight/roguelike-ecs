use specs::{Component, VecStorage};
use tcod::Color;

pub enum Arrangement {
    Foreground,
    Background,
}

pub struct Renderable {
    pub color: Color,
    pub character: Option<char>,
    pub arrangement: Arrangement,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
