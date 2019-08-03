use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Fighter {
  pub hp: i32,
  pub base_max_hp: i32,
  pub base_defense: i32,
  pub base_power: i32,
}

impl Component for Fighter {
  type Storage = VecStorage<Self>;
}
