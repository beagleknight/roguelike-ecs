use specs::{Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::WHITE;

use crate::components::{Fighter, Health, Name, Position, Velocity};
use crate::tcod::Tcod;

pub struct Combat;
impl<'a> System<'a> for Combat {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        Entities<'a>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Fighter>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (mut tcod, entity, mut health, fighter, position, name, velocity): Self::SystemData,
    ) {
        let fighters: Vec<(Entity, Fighter, Position, Name)> =
            (&entity, &fighter, &position, &name)
                .join()
                .map(|(entity, fighter, position, name)| {
                    (entity, fighter.clone(), position.clone(), name.clone())
                })
                .collect();

        for (entity, fighter, position, name, velocity) in
            (&entity, &fighter, &position, &name, &velocity).join()
        {
            for (other_entity, other_fighter, other_position, other_name) in &fighters {
                if entity != *other_entity
                    && other_position.x == position.x + velocity.x
                    && other_position.y == position.y + velocity.y
                {
                    // Do damage
                    let damage = fighter.base_power - other_fighter.base_defense;
                    tcod.log(
                        format!(
                            "{} attacks {} for {} hit points.",
                            name.name, other_name.name, damage
                        ),
                        WHITE,
                    );
                    match health.get_mut(*other_entity) {
                        Some(health) => {
                            health.hp -= damage;
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
