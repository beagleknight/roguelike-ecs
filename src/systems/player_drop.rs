use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Inventory, Object, Pickable, Player, Position};
use crate::game::{colors, Game, Turn};

pub struct PlayerDrop;

impl<'a> System<'a> for PlayerDrop {
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
        (mut game, entities, objects, position, player, mut inventories, pickable, update): Self::SystemData,
    ) {
        if game.player_turn == Turn::Drop {
            for (entity, object, position, inventory, _) in
                (&entities, &objects, &position, &mut inventories, &player).join()
            {
                // TODO: choose inventory
                if inventory.objects.len() > 0 {
                    let item_entity = inventory.objects.remove(0);
                    let item_object = objects.get(item_entity).unwrap();
                    game.log(
                        format!("You dropped a {}.", item_object.name),
                        colors::YELLOW,
                    );
                    update.insert::<Position>(item_entity, position.clone());
                }
                // if *position == player_position {
                //     game.log(format!("You picked up a {}!", object.name), colors::GREEN);
                //     update.remove::<Position>(entity);

                //     let inventory = inventories.get_mut(player_entity).unwrap();
                //     inventory.objects.push(entity);
                // }
            }
        }
    }
}
