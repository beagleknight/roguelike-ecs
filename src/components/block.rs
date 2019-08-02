use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Block;

impl Component for Block {
  type Storage = NullStorage<Self>;
}
