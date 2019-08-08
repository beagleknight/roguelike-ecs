use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Inventory, Object, Pickable, Player, Position};
use crate::game::{colors, Game, Turn};

pub struct PlayerPickUp;

impl<'a> System<'a> for PlayerPickUp {
    type SystemData = (
        WriteExpect<'a, Game>,
        Entities<'a>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, Pickable>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut game, entities, object, position, player, mut inventories, pickable, update): Self::SystemData,
    ) {
        if game.player_turn == Turn::PickUp {
            let player_components = (&entities, &position, &player)
                .join()
                .map(|(entity, position, _)| (entity, position.clone()))
                .nth(0);

            if let Some((player_entity, player_position)) = player_components {
                for (entity, object, position, _) in
                    (&entities, &object, &position, &pickable).join()
                {
                    if *position == player_position {
                        game.log(format!("You picked up a {}!", object.name), colors::GREEN);
                        update.remove::<Position>(entity);

                        let inventory = inventories.get_mut(player_entity).unwrap();
                        inventory.objects.push(entity);
                    }
                }
            }
        }
    }
}
