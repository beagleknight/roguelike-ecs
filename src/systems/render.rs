use crate::components::{Position, Renderable};
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
  );

  fn run(&mut self, (mut root, renderables, positionables): Self::SystemData) {
    root.clear();
    for (renderable, position) in (&renderables, &positionables).join() {
      root.set_default_foreground(renderable.color);
      root.put_char(
        position.x,
        position.y,
        renderable.character,
        BackgroundFlag::None,
      );
    }
    root.flush();
  }
}
