use specs::prelude::*;
use tcod::colors::WHITE;

use crate::components::{Block, Fighter, Health, Object, Player as PlayerComponent, Velocity, Inventory};
use crate::map::Map;

pub struct Player;

impl Player {
    pub fn build_entity(world: &mut World, map: &Map) {
        world
            .create_entity()
            .with(PlayerComponent)
            .with(Object {
                name: String::from("player"),
                color: WHITE,
                character: '@',
            })
            .with(Health {
                hp: 100,
                base_max_hp: 100,
            })
            .with(Fighter {
                base_defense: 1,
                base_power: 4,
            })
            .with(Inventory { objects: vec![] })
            .with(map.player_starting_position.clone())
            .with(Velocity { x: 0, y: 0 })
            .with(Block)
            .build();
    }
}
