use crate::components::{Damage, Fighter, Position, Velocity};
use specs::{Entities, Entity};
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct Combat;
impl<'a> System<'a> for Combat {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Fighter>,
    WriteStorage<'a, Damage>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Velocity>,
  );

  fn run(&mut self, (entity, mut fighter, mut damages, position, velocity): Self::SystemData) {
    let fighters: Vec<(Entity, Fighter, Position)> = (&entity, &fighter, &position)
      .join()
      .map(|(entity, fighter, position)| (entity, fighter.clone(), position.clone()))
      .collect();

    for (entity, fighter, position, velocity) in
      (&entity, &mut fighter, &position, &velocity).join()
    {
      for (other_fighter_entity, other_fighter, other_position) in &fighters {
        if entity.id() != other_fighter_entity.id()
          && other_position.x == position.x + velocity.x
          && other_position.y == position.y + velocity.y
        {
          damages
            .insert(
              *other_fighter_entity,
              Damage {
                base: fighter.base_power - other_fighter.base_defense,
              },
            )
            .unwrap();
        }
      }
    }
  }
}
