use crate::components::{Block, Player, Position};
use specs::{Join, Read, ReadStorage, System, WriteStorage};
use tcod::input::Key;
use tcod::input::KeyCode::{Down, Left, Right, Up};

pub struct PlayerMove;
impl<'a> System<'a> for PlayerMove {
  type SystemData = (
    Read<'a, Key>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, Block>,
  );

  fn run(&mut self, (key, mut positionables, players, blocks): Self::SystemData) {
    let occupied_positions: Vec<Position> = (&mut positionables, !&players, &blocks)
      .join()
      .map({ |(position, _, _)| position.clone() })
      .collect();

    for (position, _) in (&mut positionables, &players).join() {
      let movement = match *key {
        Key { code: Up, .. } => (0, -1),
        Key { code: Down, .. } => (0, 1),
        Key { code: Left, .. } => (-1, 0),
        Key { code: Right, .. } => (1, 0),
        _ => (0, 0),
      };

      for occupied_position in &occupied_positions {
        if occupied_position.x == position.x + movement.0
          && occupied_position.y == position.y + movement.1
        {
          return;
        }
      }
      position.x += movement.0;
      position.y += movement.1;
    }
  }
}
