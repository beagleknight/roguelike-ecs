use specs::{Join, LazyUpdate, Read, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Equipable, Equipment, Inventory, Object, Player, Position};
use crate::game::{colors, Game, Turn};

pub struct PlayerDrop;

impl<'a> System<'a> for PlayerDrop {
    type SystemData = (
        WriteExpect<'a, Game>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Equipable>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Equipment>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut game, objects, position, player, equipables, mut inventories, mut equipments, update): Self::SystemData,
    ) {
        if let Turn::Drop(inventory_index) = game.player_turn {
            for (position, inventory, equipment, _) in
                (&position, &mut inventories, &mut equipments, &player).join()
            {
                let item_entity = inventory.objects.remove(inventory_index);
                let item_object = objects.get(item_entity).unwrap();

                if let Some(item_equipable) = equipables.get(item_entity) {
                    if equipment.has_equiped(item_entity, item_equipable) {
                        equipment.dequip(item_equipable);
                    }
                }

                game.log(
                    format!("You dropped a {}.", item_object.name),
                    colors::YELLOW,
                );

                update.insert::<Position>(item_entity, position.clone());
            }
        }
    }
}
