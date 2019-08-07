mod components;
mod map;
mod monster;
mod player;
mod systems;
mod tcod;

use specs::prelude::*;

use crate::components::{Player as PlayerComponent, Position};
use crate::map::{FovMap, Map};
use crate::monster::Monster;
use crate::player::Player;
use crate::systems::*;
use crate::tcod::{Tcod, Turn};

fn main() {
    let tcod = Tcod::create();
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(Explore, "explore", &[])
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
    let fov_map = map.recompute_fov(&map.player_starting_position.clone());

    create_player(&mut world, &map);
    create_monsters(&mut world, &mut map);

    world.insert(tcod);
    world.insert(fov_map);

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

        let player = world.read_storage::<PlayerComponent>();
        let position = world.read_storage::<Position>();

        for (_, position) in (&player, &position).join() {
            let mut fov_map = world.write_resource::<FovMap>();
            *fov_map = map.recompute_fov(position);
        }
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
