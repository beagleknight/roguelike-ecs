use specs::{Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::WHITE;

use crate::components::{Fighter, Health, Object, Player, Position, Velocity};
use crate::tcod::Tcod;

pub struct PlayerCombat;
impl<'a> System<'a> for PlayerCombat {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        Entities<'a>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Fighter>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (mut tcod, entity, mut health, fighter, position, object, velocity, player): Self::SystemData,
    ) {
        let fighters: Vec<(Entity, Fighter, Position, Object)> =
            (&entity, &fighter, &position, &object, !&player)
                .join()
                .map(|(entity, fighter, position, object, _)| {
                    (entity, fighter.clone(), position.clone(), object.clone())
                })
                .collect();

        for (fighter, position, object, velocity, _) in
            (&fighter, &position, &object, &velocity, &player).join()
        {
            for (other_entity, other_fighter, other_position, other_object) in &fighters {
                if *other_position == position.clone() + velocity.clone() {
                    let damage = fighter.base_power - other_fighter.base_defense;
                    tcod.log(
                        format!(
                            "{} attacks {} for {} hit points.",
                            object.name, other_object.name, damage
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
