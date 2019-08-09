use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Equipable, Equipment, Inventory, Object, Player};
use crate::game::{colors, Game, Turn};

pub struct PlayerEquip;

impl<'a> System<'a> for PlayerEquip {
    type SystemData = (
        WriteExpect<'a, Game>,
        WriteStorage<'a, Equipment>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Equipable>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Inventory>,
    );

    fn run(
        &mut self,
        (mut game, mut equipment, objects, equipables, player, inventories): Self::SystemData,
    ) {
        if let Turn::Use(inventory_index) = game.player_turn {
            for (equipment, inventory, _) in (&mut equipment, &inventories, &player).join() {
                let item_entity = inventory.objects[inventory_index];
                if let Some(item_equipable) = equipables.get(item_entity) {
                    let item_object = objects.get(item_entity).unwrap();

                    if equipment.has_equiped(item_entity, item_equipable) {
                        equipment.dequip(item_equipable);
                        game.log(
                            format!(
                                "Dequipped {} from {}.",
                                item_object.name, item_equipable.slot
                            ),
                            colors::LIGHT_YELLOW,
                        );
                    } else {
                        equipment.equip(item_entity, item_equipable);
                        game.log(
                            format!("Equipped {} on {}.", item_object.name, item_equipable.slot),
                            colors::LIGHT_GREEN,
                        );
                    }
                }
            }
        }
    }
}
