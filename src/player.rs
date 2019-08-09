use specs::prelude::*;
use tcod::colors::WHITE;

use crate::components::{
    Block, Fighter, Health, Inventory, Object, Player as PlayerComponent, Velocity, Equipment
};
use crate::components::equipment::Slot;
use crate::map::Map;
use crate::item::SlotKind;

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
            .with(Equipment { slots: vec![
                Slot {
                    kind: SlotKind::LeftHand,
                    object: None
                },
                Slot {
                    kind: SlotKind::RightHand,
                    object: None
                },
                Slot {
                    kind: SlotKind::Head,
                    object: None
                }
            ] })
            .with(map.player_starting_position.clone())
            .with(Velocity { x: 0, y: 0 })
            .with(Block)
            .build();
    }
}
