use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Health {
  pub hp: i32,
  pub base_max_hp: i32,
}

impl Component for Health {
  type Storage = VecStorage<Self>;
}
