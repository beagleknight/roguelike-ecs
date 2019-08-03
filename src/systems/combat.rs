use crate::components::{Fighter, Position, Velocity};
use specs::Entities;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct Combat;
impl<'a> System<'a> for Combat {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Fighter>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Velocity>,
  );

  fn run(&mut self, (entity, mut fighter, position, velocity): Self::SystemData) {
    let fighters: Vec<(u32, Fighter, Position)> = (&entity, &fighter, &position)
      .join()
      .map(|(entity, fighter, position)| (entity.id(), fighter.clone(), position.clone()))
      .collect();

    for (entity, fighter, position, velocity) in (&entity, &mut fighter, &position, &velocity).join() {
      for (other_fighter_id, other_fighter, other_position) in &fighters {
        if entity.id() != *other_fighter_id
          && other_position.x == position.x + velocity.x
          && other_position.y == position.y + velocity.y
        {
          println!("{:?} vs {:?}", fighter, other_fighter);
        }
      }
    }
  }
}
