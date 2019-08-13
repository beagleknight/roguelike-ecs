use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

use crate::components::*;
use crate::game::{Game, Menu};
use crate::map::{DungeonLevel, FovMap, TileVisibility};

const INVENTORY_WIDTH: i32 = 50;

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        WriteExpect<'a, Game>,
        ReadExpect<'a, FovMap>,
        ReadExpect<'a, DungeonLevel>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Pickable>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Equipable>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, Corpse>,
        ReadStorage<'a, Fighter>,
        ReadStorage<'a, Experience>,
    );

    fn run(
        &mut self,
        (
            mut game,
            fov_map,
            dungeon_level,
            objects,
            tile,
            position,
            player,
            health,
            pickable,
            inventory,
            equipables,
            equipment,
            corpses,
            fighter,
            experience,
        ): Self::SystemData,
    ) {
        game.clear_window();

        for (tile, position) in (&tile, &position).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_tile(tile, position, is_in_fov);
        }

        for (object, position, _) in (&objects, &position, &corpses).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, _) in (&objects, &position, &pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, _, _) in (&objects, &position, !&player, !&pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, health, experience, _) in
            (&objects, &position, &health, &experience, &player).join()
        {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;

            game.render_object(object, position, is_in_fov);
            game.render_player_level(experience.level);
            game.render_health_bar(health.hp, health.base_max_hp);
            game.render_experience_bar(experience.points as i32, experience.next_level_points as i32);
        }

        let DungeonLevel(level) = *dungeon_level;
        game.render_dungeon_level(level);
        game.render_log();

        for (health, fighter, inventory, equipment, _) in
            (&health, &fighter, &inventory, &equipment, &player).join()
        {
            if game.menu.is_some() {
                let header = match game.menu {
                    Some(Menu::DropItem) => {
                        "Press the key next to an item to drop it, or any other to cancel.\n"
                    }
                    Some(Menu::UseItem) => {
                        "Press the key next to an item to use it, or any other to cancel.\n"
                    }
                    Some(Menu::LevelUp) => "Level up! Choose a stat to raise:\n",
                    None => unreachable!(),
                };
                let options: Vec<String> = match game.menu {
                    Some(Menu::DropItem) | Some(Menu::UseItem) => inventory
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
                        .collect(),
                    Some(Menu::LevelUp) => vec![
                        format!("Constitution (+20 HP, from {})", health.base_max_hp),
                        format!("Strength (+1 attack, from {})", fighter.base_power),
                        format!("Agility (+1 defense, from {})", fighter.base_defense),
                    ],
                    None => unreachable!(),
                };
                game.render_menu(header, &options, INVENTORY_WIDTH);
            }
        }

        game.flush();
    }
}
