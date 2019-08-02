use crate::components::{player::Player, position::Position};
use specs::{Join, Read, ReadStorage, System, WriteStorage};
use tcod::input::Key;
use tcod::input::KeyCode::{Down, Left, Right, Up};

pub struct PlayerMove;
impl<'a> System<'a> for PlayerMove {
  type SystemData = (
    Read<'a, Key>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
  );

  fn run(&mut self, (key, mut positionables, players): Self::SystemData) {
    for (position, _) in (&mut positionables, &players).join() {
      match *key {
        Key { code: Up, .. } => {
          position.y -= 1;
        }
        Key { code: Down, .. } => {
          position.y += 1;
        }
        Key { code: Left, .. } => {
          position.x -= 1;
        }
        Key { code: Right, .. } => {
          position.x += 1;
        }
        _ => {}
      }
    }
  }
}
