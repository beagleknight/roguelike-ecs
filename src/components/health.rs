use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Health {
    pub hp: i32,
    pub base_max_hp: i32,
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}

impl Health {
    pub fn heal(&mut self, amount: i32) {
        self.hp += amount;
        if self.hp > self.base_max_hp {
            self.hp = self.base_max_hp;
        }
    }
}
