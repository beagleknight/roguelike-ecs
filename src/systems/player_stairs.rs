use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Health, Player, Position, Stairs};
use crate::game::{colors, Game, Turn};

pub struct PlayerStairs;

impl<'a> System<'a> for PlayerStairs {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Stairs>,
        WriteStorage<'a, Health>,
    );

    fn run(
        &mut self,
        (mut game, entities, position, player, stairs, mut health): Self::SystemData,
    ) {
        if let Turn::Stairs(None) = game.player_turn {
            let player_components = (&entities, &position, &player)
                .join()
                .map(|(entity, position, _)| (entity, position.clone()))
                .nth(0);

            if let Some((player_entity, player_position)) = player_components {
                for (position, stairs) in (&position, &stairs).join() {
                    if *position == player_position {
                        game.log(
                            "You take a moment to rest, and recover your strength.",
                            colors::VIOLET,
                        );
                        game.player_turn = Turn::Stairs(Some(stairs.to_level));
                        let health = health.get_mut(player_entity).unwrap();
                        let heal_hp = health.base_max_hp / 2;
                        health.heal(heal_hp);
                        game.log(
                            "After a rare moment of peace, you descend deeper into \
                            the heart of the dungeon...",
                            colors::RED,
                        );
                        return;
                    }
                }
            }

            game.player_turn = Turn::Nothing;
        }
    }
}
