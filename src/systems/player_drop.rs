use specs::{Join, LazyUpdate, Read, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Inventory, Object, Player, Position};
use crate::game::{colors, Game, Turn};

pub struct PlayerDrop;

impl<'a> System<'a> for PlayerDrop {
    type SystemData = (
        WriteExpect<'a, Game>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Inventory>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut game, objects, position, player, mut inventories, update): Self::SystemData,
    ) {
        if let Turn::Drop(inventory_index) = game.player_turn {
            for (position, inventory, _) in (&position, &mut inventories, &player).join() {
                let item_entity = inventory.objects.remove(inventory_index);
                let item_object = objects.get(item_entity).unwrap();
                game.log(
                    format!("You dropped a {}.", item_object.name),
                    colors::YELLOW,
                );
                update.insert::<Position>(item_entity, position.clone());
            }
        }
    }
}
