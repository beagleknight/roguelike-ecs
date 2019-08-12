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
use crate::map::{FovMap, Map, TileVisibility, MAP_HEIGHT, MAP_WIDTH};
use crate::monster::Monster;
use crate::player::Player;
use crate::systems::*;

fn main() {
    let game = Game::create();
    let fov_map = vec![vec![TileVisibility::NotVisible; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(Explore, "explore", &[])
        .with(PlayerAction, "player_action", &[])
        .with(PlayerPickUp, "player_pick_up", &["player_action"])
        .with(PlayerDrop, "player_drop", &["player_action"])
        .with(PlayerUse, "player_use", &["player_action"])
        .with(PlayerCombat, "player_combat", &["player_action"])
        .with(PlayerStairs, "player_stairs", &["player_action"])
        .with(PlayerMovement, "player_movement", &["player_combat"])
        .with(AIVelocity, "ai_velocity", &["player_movement"])
        .with(AICombat, "ai_combat", &["ai_velocity"])
        .with(AIMovement, "ai_movement", &["ai_combat"])
        .with(Death, "death", &["ai_movement"])
        .with_thread_local(Render)
        .build();

    dispatcher.setup(&mut world);

    create_player(&mut world);

    let mut map = create_map(1, &mut world);

    world.insert(game);
    world.insert(fov_map);

    loop {
        dispatcher.dispatch(&mut world);
        world.maintain();

        let player_turn = {
            let game = world.read_resource::<Game>();
            game.player_turn.clone()
        };

        if let Turn::Stairs(Some(level)) = player_turn {
            map = create_map(level, &mut world);
        }

        recompute_fov(&world, &mut map);

        let mut game = world.write_resource::<Game>();
        game.read_input();
        if game.exit() {
            break;
        }
    }
}

fn create_map(level: u32, world: &mut World) -> Map {
    let mut map = Map::create(level);
    map.build_entities(world);
    place_player(world, &map);
    create_monsters(world, &mut map);
    create_items(world, &mut map);
    map
}

fn create_player(world: &mut World) {
    Player::build_entity(world);
}

fn place_player(world: &mut World, map: &Map) {
    let player = world.read_storage::<PlayerComponent>();
    let mut position = world.write_storage::<Position>();

    for (_, position) in (&player, &mut position).join() {
        *position = map.player_starting_position.clone();
    }
}

fn create_monsters(world: &mut World, map: &mut Map) {
    let monsters = Monster::place_monsters(map);
    Monster::build_entities(monsters, world);
}

fn create_items(world: &mut World, map: &mut Map) {
    let items = Item::place_items(map);
    Item::build_entities(items, world);
}

fn recompute_fov(world: &World, map: &mut Map) {
    let player = world.read_storage::<PlayerComponent>();
    let position = world.read_storage::<Position>();

    for (_, position) in (&player, &position).join() {
        let mut fov_map = world.write_resource::<FovMap>();
        *fov_map = map.recompute_fov(position);
    }
}
