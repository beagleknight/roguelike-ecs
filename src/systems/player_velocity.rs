use crate::components::{Player, Velocity};
use specs::{Join, Read, ReadStorage, System, WriteStorage};
use tcod::input::{
  Key,
  KeyCode::{Down, Left, Right, Up},
};

pub struct PlayerVelocity;
impl<'a> System<'a> for PlayerVelocity {
  type SystemData = (
    Read<'a, Key>,
    WriteStorage<'a, Velocity>,
    ReadStorage<'a, Player>,
  );

  fn run(&mut self, (key, mut velocity, player): Self::SystemData) {
    for (velocity, _) in (&mut velocity, &player).join() {
      let input_velocity = match *key {
        Key { code: Up, .. } => Velocity { x: 0, y: -1 },
        Key { code: Down, .. } => Velocity { x: 0, y: 1 },
        Key { code: Left, .. } => Velocity { x: -1, y: 0 },
        Key { code: Right, .. } => Velocity { x: 1, y: 0 },
        _ => Velocity { x: 0, y: 0 },
      };
      *velocity = input_velocity;
    }
  }
}
