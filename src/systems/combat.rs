use specs::{Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::WHITE;

use crate::components::{Fighter, Health, Position, Velocity};
use crate::tcod::Tcod;

pub struct Combat;
impl<'a> System<'a> for Combat {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        Entities<'a>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Fighter>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (mut tcod, entity, mut health, fighter, position, velocity): Self::SystemData,
    ) {
        let fighters: Vec<(Entity, Fighter, Position)> = (&entity, &fighter, &position)
            .join()
            .map(|(entity, fighter, position)| (entity, fighter.clone(), position.clone()))
            .collect();

        for (entity, fighter, position, velocity) in
            (&entity, &fighter, &position, &velocity).join()
        {
            for (other_entity, other_fighter, other_position) in &fighters {
                if entity.id() != other_entity.id()
                    && other_position.x == position.x + velocity.x
                    && other_position.y == position.y + velocity.y
                {
                    // Do damage
                    let damage = fighter.base_power - other_fighter.base_defense;
                    tcod.log(
                        format!(
                            "{} attacks {} for {} hit points.",
                            entity.id(),
                            other_entity.id(),
                            damage
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
