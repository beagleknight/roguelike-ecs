use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};
use tcod::console::Console;

use crate::components::{Object, Player, Position, Tile};
use crate::map::{FovMap, TileVisibility};
use crate::tcod::Tcod;

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        ReadExpect<'a, FovMap>,
        ReadStorage<'a, Object>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut tcod, fov_map, object, tile, position, player): Self::SystemData) {
        tcod.root.clear();

        for (object, position, _) in (&object, &position, !&player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            tcod.render_object(object, position, is_in_fov);
        }

        for (object, position, _) in (&object, &position, &player).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            tcod.render_object(object, position, is_in_fov);
        }

        for (tile, position) in (&tile, &position).join() {
            let is_in_fov =
                fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible;
            tcod.render_tile(tile, position, is_in_fov);
        }

        tcod.render_log();
        tcod.root.flush();
    }
}
