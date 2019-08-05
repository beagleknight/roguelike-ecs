use specs::{Entities, Join, ReadStorage, System, WriteStorage};
use tcod::colors::DARK_RED;

use crate::components::renderable::Arrangement;
use crate::components::{Health, Player, Position, Renderable};

pub struct Death;
impl<'a> System<'a> for Death {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, (entities, health, player, mut positions, mut renderable): Self::SystemData) {
        for (entity, health, _) in (&entities, &health, !&player).join() {
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

                entities.delete(entity).unwrap();
            }
        }
    }
}
