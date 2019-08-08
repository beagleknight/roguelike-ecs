use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Health, Object, Player, Position};
use crate::game::{colors, Game};

pub struct Death;
impl<'a> System<'a> for Death {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Object>,
    );

    fn run(
        &mut self,
        (mut game, entities, health, player, mut positions, mut objects): Self::SystemData,
    ) {
        let mut corpses_positions: Vec<Position> = vec![];

        for (entity, health, object, position, _) in
            (&entities, &health, &objects, &positions, !&player).join()
        {
            if health.hp <= 0 {
                corpses_positions.push(position.clone());
                game.log(format!("{} is dead!", object.name), colors::ORANGE);
                entities.delete(entity).unwrap();
            }
        }

        for (entity, health, position, _) in (&entities, &health, &positions, &player).join() {
            if health.hp <= 0 {
                corpses_positions.push(position.clone());
                game.log(format!("You died!"), colors::RED);
                entities.delete(entity).unwrap();
            }
        }

        for corpse_position in &corpses_positions {
            entities
                .build_entity()
                .with(
                    Object {
                        name: String::from("corpse"),
                        color: colors::DARK_RED,
                        character: '%',
                    },
                    &mut objects,
                )
                .with(corpse_position.clone(), &mut positions)
                .build();
        }
    }
}
