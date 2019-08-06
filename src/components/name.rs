use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
}

impl Component for Name {
    type Storage = VecStorage<Self>;
}
