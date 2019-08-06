use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::colors::DARK_RED;
use tcod::colors::ORANGE;

use crate::components::renderable::Arrangement;
use crate::components::{Health, Name, Player, Position, Renderable};
use crate::tcod::Tcod;

pub struct Death;
impl<'a> System<'a> for Death {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Renderable>,
    );

    fn run(
        &mut self,
        (mut tcod, entities, health, player, mut positions, name, mut renderable): Self::SystemData,
    ) {
        for (entity, health, name, _) in (&entities, &health, &name, !&player).join() {
            if health.hp <= 0 {
                let position = positions.get_mut(entity).unwrap();

                entities
                    .build_entity()
                    .with(
                        Renderable {
                            color: DARK_RED,
                            character: Some('%'),
                            arrangement: Arrangement::Foreground,
                        },
                        &mut renderable,
                    )
                    .with(position.clone(), &mut positions)
                    .build();

                tcod.log(
                    format!("{} is dead! You gain {} experience points.", name.name, 0),
                    ORANGE,
                );

                entities.delete(entity).unwrap();
            }
        }
    }
}