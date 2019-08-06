mod components;
mod map;
mod monster;
mod player;
mod systems;
mod tcod;

use specs::prelude::*;

use crate::map::Map;
use crate::monster::Monster;
use crate::player::Player;
use crate::systems::{AIVelocity, PlayerCombat, AICombat, Death, PlayerMovement, AIMovement, PlayerVelocity, Render};
use crate::tcod::{Tcod, Turn};

fn main() {
    let tcod = Tcod::create();
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerVelocity, "player_velocity", &[])
        .with(PlayerCombat, "player_combat", &["player_velocity"])
        .with(PlayerMovement, "player_movement", &["player_combat"])
        .with(AIVelocity, "ai_velocity", &["player_movement"])
        .with(AICombat, "ai_combat", &["ai_velocity"])
        .with(AIMovement, "ai_movement", &["ai_combat"])
        .with(Death, "death", &["ai_movement"])
        .with_thread_local(Render)
        .build();

    dispatcher.setup(&mut world);

    let mut map = create_map(&mut world);
    create_player(&mut world, &map);
    create_monsters(&mut world, &mut map);

    world.insert(tcod);

    loop {
        dispatcher.dispatch(&mut world);
        world.maintain();

        let key = Tcod::read_key();
        if Tcod::exit(key) {
            break;
        }

        let mut tcod = world.write_resource::<Tcod>();
        tcod.key = key;
        tcod.player_turn = Turn::Nothing;
    }
}

fn create_map(world: &mut World) -> Map {
    let map = Map::create();
    map.build_entities(world);
    map
}

fn create_player(world: &mut World, map: &Map) {
    Player::build_entity(world, map);
}

fn create_monsters(world: &mut World, map: &mut Map) {
    let monsters = Monster::place_monsters(map);
    Monster::build_entities(monsters, world);
}
