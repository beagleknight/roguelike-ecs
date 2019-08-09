use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

use crate::components::{
    Equipable, Equipment, Health, Inventory, Object, Pickable, Player, Position, Tile,
};
use crate::game::{Game, InventoryAction};
use crate::map::{FovMap, TileVisibility};

const INVENTORY_WIDTH: i32 = 50;

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        WriteExpect<'a, Game>,
        ReadExpect<'a, FovMap>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Pickable>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Equipable>,
        ReadStorage<'a, Equipment>,
    );

    fn run(
        &mut self,
        (
            mut game,
            fov_map,
            objects,
            tile,
            position,
            player,
            health,
            pickable,
            inventory,
            equipables,
            equipment,
        ): Self::SystemData,
    ) {
        game.clear_window();

        for (tile, position) in (&tile, &position).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_tile(tile, position, is_in_fov);
        }

        for (object, position, _, _) in (&objects, &position, !&player, &pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, _, _) in (&objects, &position, !&player, !&pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, health, _) in (&objects, &position, &health, &player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;

            game.render_object(object, position, is_in_fov);
            game.render_health_bar(health.hp, health.base_max_hp);
        }

        game.render_log();

        for (inventory, equipment, _) in (&inventory, &equipment, &player).join() {
            if game.inventory_action.is_some() {
                let object_names: Vec<String> = inventory
                    .objects
                    .iter()
                    .map(|&item_entity| {
                        let item_object = objects.get(item_entity).unwrap();
                        let item_equipable = equipables.get(item_entity);

                        if let Some(item_equipable) = item_equipable {
                            if equipment.has_equiped(item_entity, item_equipable) {
                                return format!(
                                    "{} (on {})",
                                    item_object.name, item_equipable.slot
                                );
                            }
                        }
                        item_object.name.clone()
                    })
                    .collect();
                let header = match game.inventory_action {
                    Some(InventoryAction::Drop) => {
                        "Press the key next to an item to drop it, or any other to cancel.\n"
                    }
                    Some(InventoryAction::Use) => {
                        "Press the key next to an item to use it, or any other to cancel.\n"
                    }
                    None => unreachable!(),
                };
                game.show_inventory_menu(header, &object_names, INVENTORY_WIDTH);
            }
        }

        game.flush();
    }
}
