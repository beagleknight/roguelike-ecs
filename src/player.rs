use specs::prelude::*;
use tcod::colors::WHITE;

use crate::components::equipment::Slot;
use crate::components::{Player as PlayerComponent, *};
use crate::item::SlotKind;

pub const LEVEL_UP_BASE: u32 = 200;
pub const LEVEL_UP_FACTOR: u32 = 150;

pub struct Player;

impl Player {
    pub fn build_entity(world: &mut World) {
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
            .with(Experience {
                level: 1,
                points: 0,
                next_level_points: Some(LEVEL_UP_BASE + LEVEL_UP_FACTOR),
            })
            .with(Inventory { objects: vec![] })
            .with(Equipment {
                slots: vec![
                    Slot {
                        kind: SlotKind::LeftHand,
                        object: None,
                    },
                    Slot {
                        kind: SlotKind::RightHand,
                        object: None,
                    },
                    Slot {
                        kind: SlotKind::Head,
                        object: None,
                    },
                ],
            })
            .with(Position { x: 0, y: 0 })
            .with(Velocity { x: 0, y: 0 })
            .with(Block)
            .build();
    }
}
