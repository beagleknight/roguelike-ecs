mod components;
mod map;
mod monster;
mod systems;

use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::{self, Event, KeyCode};

use crate::components::renderable::Arrangement;
use crate::components::{Block, Fighter, Player, Position, Renderable, Velocity};
use crate::map::{Map, Tile};
use crate::monster::{Monster, MonsterKind};
use crate::systems::{Combat, Movement, PlayerVelocity, Render};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

fn main() {
    let key: Key = Default::default();
    let root = Root::initializer()
        .font("terminal8x8_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial [with specs]")
        .init();

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerVelocity, "player_velocity", &[])
        .with(Combat, "combat", &["player_velocity"])
        .with(Movement, "movement", &["combat"])
        .with_thread_local(Render)
        .build();

    dispatcher.setup(&mut world);

    let mut map = create_map(&mut world);
    create_player(&mut world, &map);
    create_monsters(&mut world, &mut map);

    world.insert(key);
    world.insert(root);

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

fn create_map(world: &mut World) -> Map {
    let map = Map::create();

    for y in 0..map.height {
        for x in 0..map.width {
            match map.tiles[x as usize][y as usize] {
                Tile::Wall => {
                    world
                        .create_entity()
                        .with(Renderable {
                            color: COLOR_DARK_WALL,
                            character: None,
                            arrangement: Arrangement::Background,
                        })
                        .with(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                        .with(Block)
                        .build();
                }
                Tile::Floor => {
                    world
                        .create_entity()
                        .with(Renderable {
                            color: COLOR_DARK_GROUND,
                            character: None,
                            arrangement: Arrangement::Background,
                        })
                        .with(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                        .build();
                }
            }
        }
    }

    map
}

fn create_player(world: &mut World, map: &Map) {
    world
        .create_entity()
        .with(Player)
        .with(Renderable {
            color: WHITE,
            character: Some('@'),
            arrangement: Arrangement::Foreground,
        })
        .with(Fighter {
            hp: 100,
            base_max_hp: 100,
            base_defense: 1,
            base_power: 4,
        })
        .with(map.player_starting_position.clone())
        .with(Velocity { x: 0, y: 0 })
        .with(Block)
        .build();
}

fn create_monsters(world: &mut World, map: &mut Map) {
    let monsters = Monster::place_monsters(map);

    for monster in &monsters {
        match monster.kind {
            MonsterKind::Orc => {
                world
                    .create_entity()
                    .with(Renderable {
                        color: DESATURATED_GREEN,
                        character: Some('o'),
                        arrangement: Arrangement::Foreground,
                    })
                    .with(Fighter {
                        hp: 20,
                        base_max_hp: 20,
                        base_defense: 0,
                        base_power: 4,
                    })
                    .with(monster.position.clone())
                    .with(Velocity { x: 0, y: 0 })
                    .with(Block)
                    .build();
            }
            MonsterKind::Troll => {
                world
                    .create_entity()
                    .with(Renderable {
                        color: DARKER_GREEN,
                        character: Some('T'),
                        arrangement: Arrangement::Foreground,
                    })
                    .with(Fighter {
                        hp: 30,
                        base_max_hp: 30,
                        base_defense: 2,
                        base_power: 8,
                    })
                    .with(monster.position.clone())
                    .with(Velocity { x: 0, y: 0 })
                    .with(Block)
                    .build();
            }
        }
    }
}
