use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

use crate::components::{Health, Object, Pickable, Player, Position, Tile};
use crate::game::Game;
use crate::map::{FovMap, TileVisibility};

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
    );

    fn run(
        &mut self,
        (mut game, fov_map, object, tile, position, player, health, pickable): Self::SystemData,
    ) {
        game.clear_window();

        for (tile, position) in (&tile, &position).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_tile(tile, position, is_in_fov);
        }

        for (object, position, _, _) in (&object, &position, !&player, &pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, _, _) in (&object, &position, !&player, !&pickable).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, health, _) in (&object, &position, &health, &player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
            game.render_health_bar(health.hp, health.base_max_hp);
        }

        game.render_log();
        game.flush();
    }
}
