use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect};

use crate::components::{Object, Pickable, Player, Position};
use crate::game::{colors, Game, Turn};

pub struct PlayerPickUp;

impl<'a> System<'a> for PlayerPickUp {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pickable>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut game, entities, object, position, player, pickable, update): Self::SystemData,
    ) {
        if game.player_turn == Turn::PickUp {
            let player_position = (&position, &player)
                .join()
                .map(|(position, _)| position.clone())
                .nth(0);

            if let Some(player_position) = player_position {
                for (entity, object, position, _) in
                    (&entities, &object, &position, &pickable).join()
                {
                    if *position == player_position {
                        game.log(format!("You picked up a {}!", object.name), colors::GREEN);
                        update.remove::<Position>(entity);
                    }
                }
            }
        }
    }
}
