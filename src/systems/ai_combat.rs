use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::WHITE;

use crate::components::{Fighter, Health, Object, Player, Position, Velocity};
use crate::tcod::Tcod;

pub struct AICombat;
impl<'a> System<'a> for AICombat {
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
        let player_components = (&entity, &fighter, &position, &object, &player)
            .join()
            .map(|(entity, fighter, position, object, _)| {
                (entity, fighter.clone(), position.clone(), object.clone())
            })
            .nth(0);

        if let Some((player_entity, player_fighter, player_position, player_object)) =
            player_components
        {
            for (fighter, position, object, velocity, _) in
                (&fighter, &position, &object, &velocity, !&player).join()
            {
                if player_position == position.clone() + velocity.clone() {
                    let damage = fighter.base_power - player_fighter.base_defense;
                    tcod.log(
                        format!(
                            "{} attacks {} for {} hit points.",
                            object.name, player_object.name, damage
                        ),
                        WHITE,
                    );
                    match health.get_mut(player_entity) {
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
