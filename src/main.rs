use crate::systems::{PlayerMove, Render};
use crate::components::{Renderable, Position, Player, Block};
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::{self, Event, KeyCode};

mod components;
mod systems;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

fn main() {
    let mut world = World::new();

    world.register::<Renderable>();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Block>();

    world
        .create_entity()
        .with(Renderable {
            color: GREEN,
            character: 'o',
        })
        .with(Position { x: 30, y: 20 })
        .with(Block)
        .build();

    world
        .create_entity()
        .with(Renderable {
            color: YELLOW,
            character: '#',
        })
        .with(Position { x: 30, y: 30 })
        .build();

    world
        .create_entity()
        .with(Renderable {
            color: WHITE,
            character: '@',
        })
        .with(Position { x: 20, y: 20 })
        .with(Player)
        .build();

    let key: Key = Default::default();
    let root = Root::initializer()
        .font("terminal8x8_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial [with specs]")
        .init();

    world.insert(key);
    world.insert(root);

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerMove, "player_move", &[])
        .with_thread_local(Render)
        .build();

    loop {
        dispatcher.dispatch(&mut world);

        let mut key = world.write_resource::<Key>();
        match input::check_for_event(input::KEY_PRESS) {
            Some((_, Event::Key(k))) => {
                *key = k;
                if k.code == KeyCode::Escape {
                    break;
                }
            }
            _ => {
                *key = Default::default();
            }
        }
    }
}
