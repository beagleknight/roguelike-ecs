use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};
use tcod::{console::Console, BackgroundFlag};

use crate::components::renderable::Arrangement;
use crate::components::{Player, Position, Renderable};
use crate::map::{FovMap, TileVisibility};
use crate::tcod::Tcod;

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        ReadExpect<'a, FovMap>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut tcod, fov_map, renderables, positionables, player): Self::SystemData) {
        tcod.root.clear();

        for (renderable, position, _) in (&renderables, &positionables, !&player).join() {
            if let TileVisibility::Visible = fov_map[position.x as usize][position.y as usize] {
                match renderable.arrangement {
                    Arrangement::Foreground => {
                        tcod.root.set_default_foreground(renderable.color);
                        tcod.root.put_char(
                            position.x,
                            position.y,
                            renderable.character.unwrap(),
                            BackgroundFlag::None,
                        );
                    }
                    Arrangement::Background => {
                        tcod.root.set_char_background(
                            position.x,
                            position.y,
                            renderable.color,
                            BackgroundFlag::Set,
                        );
                    }
                }
            }
        }

        for (renderable, position, _) in (&renderables, &positionables, &player).join() {
            if let TileVisibility::Visible = fov_map[position.x as usize][position.y as usize] {
                tcod.root.set_default_foreground(renderable.color);
                tcod.root.put_char(
                    position.x,
                    position.y,
                    renderable.character.unwrap(),
                    BackgroundFlag::None,
                );
            }
        }

        tcod.render_log();
        tcod.root.flush();
    }
}
