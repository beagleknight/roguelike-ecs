use crate::components::Velocity;
use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn direction_to(&self, other: &Position) -> Velocity {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = self.distance_to(other);

        if distance > 0.0 {
            let dx = (dx as f32 / distance).round() as i32;
            let dy = (dy as f32 / distance).round() as i32;

            return Velocity { x: dx, y: dy };
        }

        Velocity { x: dx, y: dy }
    }
}
