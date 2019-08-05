use crate::components::renderable::Arrangement;
use crate::components::{Player, Position, Renderable};
use specs::{Join, ReadStorage, System, WriteExpect};
use tcod::{
  console::{Console, Root},
  BackgroundFlag,
};

pub struct Render;
impl<'a> System<'a> for Render {
  type SystemData = (
    WriteExpect<'a, Root>,
    ReadStorage<'a, Renderable>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Player>,
  );

  fn run(&mut self, (mut root, renderables, positionables, player): Self::SystemData) {
    root.clear();

    for (renderable, position, _) in (&renderables, &positionables, !&player).join() {
      match renderable.arrangement {
        Arrangement::Foreground => {
          root.set_default_foreground(renderable.color);
          root.put_char(
            position.x,
            position.y,
            renderable.character.unwrap(),
            BackgroundFlag::None,
          );
        }
        Arrangement::Background => {
          root.set_char_background(
            position.x,
            position.y,
            renderable.color,
            BackgroundFlag::Set,
          );
        }
      }
    }

    for (renderable, position, _) in (&renderables, &positionables, &player).join() {
      root.set_default_foreground(renderable.color);
      root.put_char(
        position.x,
        position.y,
        renderable.character.unwrap(),
        BackgroundFlag::None,
      );
    }

    root.flush();
  }
}
