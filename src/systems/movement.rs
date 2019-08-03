use crate::components::{Block, Position, Velocity};
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct Movement;
impl<'a> System<'a> for Movement {
  type SystemData = (
    WriteStorage<'a, Position>,
    ReadStorage<'a, Velocity>,
    ReadStorage<'a, Block>,
  );

  fn run(&mut self, (mut position, velocity, block): Self::SystemData) {
    let occupied_positions: Vec<Position> = (&mut position, &block)
      .join()
      .map({ |(position, _)| position.clone() })
      .collect();

    for (position, velocity) in (&mut position, &velocity).join() {
      for occupied_position in &occupied_positions {
        if occupied_position.x == position.x + velocity.x
          && occupied_position.y == position.y + velocity.y
        {
          return;
        }
      }

      position.x += velocity.x;
      position.y += velocity.y;
    }
  }
}
