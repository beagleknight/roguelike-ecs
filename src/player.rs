use specs::prelude::*;
use tcod::colors::WHITE;

use crate::components::renderable::Arrangement;
use crate::components::{Block, Fighter, Health, Player as PlayerComponent, Renderable, Velocity};
use crate::map::Map;

pub struct Player;

impl Player {
    pub fn build_entity(world: &mut World, map: &Map) {
        world
            .create_entity()
            .with(PlayerComponent)
            .with(Renderable {
                color: WHITE,
                character: Some('@'),
                arrangement: Arrangement::Foreground,
            })
            .with(Health {
                hp: 100,
                base_max_hp: 100,
            })
            .with(Fighter {
                base_defense: 1,
                base_power: 4,
            })
            .with(map.player_starting_position.clone())
            .with(Velocity { x: 0, y: 0 })
            .with(Block)
            .build();
    }
}
