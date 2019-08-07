use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::{Position, Tile};
use crate::map::{FovMap, TileVisibility};

pub struct Explore;
impl<'a> System<'a> for Explore {
    type SystemData = (
        ReadExpect<'a, FovMap>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Tile>,
    );

    fn run(&mut self, (fov_map, position, mut tile): Self::SystemData) {
        for (position, tile) in (&position, &mut tile).join() {
            if !tile.explored
                && fov_map[position.x as usize][position.y as usize] == TileVisibility::Visible
            {
                tile.explored = true;
            }
        }
    }
}
