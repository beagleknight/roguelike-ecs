use specs::{Component, VecStorage};
use tcod::Color;

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub color: Color,
    pub character: char,
}

impl Component for Object {
    type Storage = VecStorage<Self>;
}
