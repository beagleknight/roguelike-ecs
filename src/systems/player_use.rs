use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::*;
use crate::game::{colors, Game, Turn};
use crate::item::ItemKind;

const HEAL_AMOUNT: i32 = 40;

pub struct PlayerUse;

impl<'a> System<'a> for PlayerUse {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Game>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Equipable>,
        WriteStorage<'a, Equipment>,
        ReadStorage<'a, Usable>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Inventory>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut game,
            objects,
            equipables,
            mut equipments,
            usables,
            mut health,
            player,
            mut inventories,
        ): Self::SystemData,
    ) {
        if let Turn::Use(inventory_index) = game.player_turn {
            for (health, inventory, equipment, _) in
                (&mut health, &mut inventories, &mut equipments, &player).join()
            {
                let item_entity = inventory.objects[inventory_index];
                if let Some(item_usable) = usables.get(item_entity) {
                    inventory.objects.remove(inventory_index);
                    match item_usable.kind {
                        ItemKind::HealthPotion => {
                            game.log("Your wounds start to feel better!", colors::LIGHT_VIOLET);
                            health.hp += HEAL_AMOUNT;
                            if health.hp > health.base_max_hp {
                                health.hp = health.base_max_hp;
                            }
                        }
                        _ => unreachable!(),
                    }
                    entities.delete(item_entity).unwrap();
                } else if let Some(item_equipable) = equipables.get(item_entity) {
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
