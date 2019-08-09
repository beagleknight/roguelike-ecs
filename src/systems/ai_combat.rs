use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::*;
use crate::game::{colors, Game};

pub struct AICombat;
impl<'a> System<'a> for AICombat {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Fighter>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Equipment>,
    );

    fn run(
        &mut self,
        (mut game, entity, mut health, fighter, position, object, velocity, player, equipments): Self::SystemData,
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
            for (entity, fighter, position, object, velocity, _) in
                (&entity, &fighter, &position, &object, &velocity, !&player).join()
            {
                if player_position == position.clone() + velocity.clone() {
                    let equipment_power = equipments
                        .get(entity)
                        .map_or(0, |equipment| equipment.power());
                    let equipment_defense = equipments
                        .get(player_entity)
                        .map_or(0, |equipment| equipment.defense());
                    let power = fighter.base_power + equipment_power;
                    let defense = player_fighter.base_defense + equipment_defense;
                    let damage = power - defense;
                    game.log(
                        format!(
                            "{} attacks {} for {} hit points.",
                            object.name, player_object.name, damage
                        ),
                        colors::WHITE,
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
