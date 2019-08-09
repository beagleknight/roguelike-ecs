use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Health, Inventory, Player, Usable};
use crate::game::{colors, Game, Turn};
use crate::item::ItemKind;

const HEAL_AMOUNT: i32 = 40;

pub struct PlayerUse;

impl<'a> System<'a> for PlayerUse {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Game>,
        ReadStorage<'a, Usable>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Inventory>,
    );

    fn run(
        &mut self,
        (entities, mut game, usables, mut health, player, mut inventories): Self::SystemData,
    ) {
        if let Turn::Use(inventory_index) = game.player_turn {
            for (health, inventory, _) in (&mut health, &mut inventories, &player).join() {
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
                }
            }
        }
    }
}
