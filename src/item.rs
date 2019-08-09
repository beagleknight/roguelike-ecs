use rand::{
    distributions::{IndependentSample, Weighted, WeightedChoice},
    Rng,
};
use specs::prelude::*;

use crate::components::{Equipable, Object, Pickable, Position, Usable};
use crate::game::colors;
use crate::map::Map;

const MAX_ITEMS: i32 = 3;

#[derive(Clone, Copy, PartialEq)]
pub enum ItemKind {
    HealthPotion,
    Sword,
}

#[derive(Clone, PartialEq)]
pub enum SlotKind {
    LeftHand,
    RightHand,
    Head,
}

impl std::fmt::Display for SlotKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SlotKind::LeftHand => write!(f, "left hand"),
            SlotKind::RightHand => write!(f, "right hand"),
            SlotKind::Head => write!(f, "head"),
        }
    }
}

pub struct Item {
    pub kind: ItemKind,
    pub position: Position,
}

impl Item {
    pub fn place_items(map: &mut Map) -> Vec<Item> {
        let mut items = vec![];

        for room in &map.rooms {
            let num_items = rand::thread_rng().gen_range(0, MAX_ITEMS + 1);
            let item_chances = &mut [
                Weighted {
                    weight: 50,
                    item: ItemKind::HealthPotion,
                },
                Weighted {
                    weight: 50,
                    item: ItemKind::Sword,
                },
            ];
            let item_choice = WeightedChoice::new(item_chances);

            for _ in 0..num_items {
                let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
                let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

                if !map.is_occupied(x, y) {
                    items.push(Item {
                        kind: item_choice.ind_sample(&mut rand::thread_rng()),
                        position: Position { x, y },
                    });
                    map.occupied_places.push(Position { x, y });
                }
            }
        }

        items
    }

    pub fn build_entities(items: Vec<Item>, world: &mut World) {
        for item in &items {
            match item.kind {
                ItemKind::HealthPotion => {
                    world
                        .create_entity()
                        .with(Object {
                            name: String::from("healing potion"),
                            color: colors::VIOLET,
                            character: '!',
                        })
                        .with(item.position.clone())
                        .with(Pickable)
                        .with(Usable {
                            kind: ItemKind::HealthPotion,
                        })
                        .build();
                }
                ItemKind::Sword => {
                    world
                        .create_entity()
                        .with(Object {
                            name: String::from("sword"),
                            color: colors::SKY,
                            character: '/',
                        })
                        .with(item.position.clone())
                        .with(Pickable)
                        .with(Equipable {
                            max_hp_bonus: 0,
                            power_bonus: 3,
                            defense_bonus: 0,
                            slot: SlotKind::RightHand,
                        })
                        .build();
                }
            }
        }
    }
}
