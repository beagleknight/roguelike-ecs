mod components;
mod game;
mod item;
mod map;
mod monster;
mod player;
mod systems;

use specs::prelude::*;

use crate::components::{Player as PlayerComponent, Position};
use crate::game::{Game, Turn};
use crate::item::Item;
use crate::map::{FovMap, Map};
use crate::monster::Monster;
use crate::player::Player;
use crate::systems::*;

fn main() {
    let game = Game::create();
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(Explore, "explore", &[])
        .with(PlayerAction, "player_action", &[])
        .with(PlayerPickUp, "player_pick_up", &["player_action"])
        .with(PlayerDrop, "player_drop", &["player_action"])
        .with(PlayerUse, "player_use", &["player_action"])
        .with(PlayerCombat, "player_combat", &["player_action"])
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
    create_items(&mut world, &mut map);

    world.insert(game);
    world.insert(fov_map);

    loop {
        dispatcher.dispatch(&mut world);
        world.maintain();

        let mut game = world.write_resource::<Game>();
        let player = world.read_storage::<PlayerComponent>();
        let position = world.read_storage::<Position>();

        for (_, position) in (&player, &position).join() {
            let mut fov_map = world.write_resource::<FovMap>();
            *fov_map = map.recompute_fov(position);
        }

        let key = Game::read_key();
        if game.exit(key) {
            break;
        }
        game.key = key;
        game.player_turn = Turn::Nothing;
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

fn create_items(world: &mut World, map: &mut Map) {
    let items = Item::place_items(map);
    Item::build_entities(items, world);
}
