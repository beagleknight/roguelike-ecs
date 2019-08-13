use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::*;
use crate::game::{colors, Game};

pub struct Death;
impl<'a> System<'a> for Death {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Object>,
        WriteStorage<'a, Corpse>,
    );

    fn run(
        &mut self,
        (
            mut game,
            entities,
            health,
            player,
            mut experiences,
            mut positions,
            mut objects,
            mut corpses,
        ): Self::SystemData,
    ) {
        let mut total_experience_won = 0;
        let mut corpses_positions: Vec<Position> = vec![];

        for (entity, health, object, position, experience, _) in (
            &entities,
            &health,
            &objects,
            &positions,
            &experiences,
            !&player,
        )
            .join()
        {
            if health.hp <= 0 {
                corpses_positions.push(position.clone());
                game.log(
                    format!(
                        "{} is dead! You gain {} experience points.",
                        object.name,
                        experience.level * experience.points
                    ),
                    colors::ORANGE,
                );
                total_experience_won += experience.level * experience.points;
                entities.delete(entity).unwrap();
            }
        }

        for (entity, health, position, experience, _) in
            (&entities, &health, &positions, &mut experiences, &player).join()
        {
            if health.hp <= 0 {
                corpses_positions.push(position.clone());
                game.log(format!("You died!"), colors::RED);
                entities.delete(entity).unwrap();
            } else {
                experience.points += total_experience_won;
            }
        }

        for corpse_position in &corpses_positions {
            entities
                .build_entity()
                .with(Corpse, &mut corpses)
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
