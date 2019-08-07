use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

use crate::components::{Object, Player, Position, Tile};
use crate::map::{FovMap, TileVisibility};
use crate::game::Game;

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        WriteExpect<'a, Game>,
        ReadExpect<'a, FovMap>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut game, fov_map, object, tile, position, player): Self::SystemData) {
        game.clear_window();

        for (object, position, _) in (&object, &position, !&player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (object, position, _) in (&object, &position, &player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_object(object, position, is_in_fov);
        }

        for (tile, position) in (&tile, &position).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            game.render_tile(tile, position, is_in_fov);
        }

        game.render_log();
        game.flush();
    }
}
