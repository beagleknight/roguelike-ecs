use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::DARK_RED;
use tcod::colors::ORANGE;

use crate::components::{Health, Object, Player, Position};
use crate::tcod::Tcod;

pub struct Death;
impl<'a> System<'a> for Death {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Object>,
    );

    fn run(
        &mut self,
        (mut tcod, entities, health, player, mut positions, mut objects): Self::SystemData,
    ) {
        let mut corpses_positions: Vec<Position> = vec![];

        for (entity, health, object, position, _) in
            (&entities, &health, &mut objects, &positions, !&player).join()
        {
            if health.hp <= 0 {
                corpses_positions.push(position.clone());
                tcod.log(
                    format!("{} is dead! You gain {} experience points.", object.name, 0),
                    ORANGE,
                );
                entities.delete(entity).unwrap();
            }
        }

        for corpse_position in &corpses_positions {
            entities
                .build_entity()
                .with(
                    Object {
                        name: String::from("corpse"),
                        color: DARK_RED,
                        character: '%',
                    },
                    &mut objects,
                )
                .with(corpse_position.clone(), &mut positions)
                .build();
        }
    }
}
