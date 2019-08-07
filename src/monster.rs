use rand::{
    distributions::{IndependentSample, Weighted, WeightedChoice},
    Rng,
};
use specs::prelude::*;
use tcod::colors::{DARKER_GREEN, DESATURATED_GREEN};

use crate::components::{AIControlled, Block, Fighter, Health, Object, Position, Velocity};
use crate::map::Map;

const MAX_MONSTERS: i32 = 3;

#[derive(Clone)]
pub enum MonsterKind {
    Orc,
    Troll,
}

pub struct Monster {
    pub kind: MonsterKind,
    pub position: Position,
}

impl Monster {
    pub fn place_monsters(map: &mut Map) -> Vec<Monster> {
        let mut monsters = vec![];

        for room in &map.rooms {
            let num_monsters = rand::thread_rng().gen_range(0, MAX_MONSTERS + 1);
            let monster_chances = &mut [
                Weighted {
                    weight: 80,
                    item: MonsterKind::Orc,
                },
                Weighted {
                    weight: 20,
                    item: MonsterKind::Troll,
                },
            ];
            let monster_choice = WeightedChoice::new(monster_chances);

            for _ in 0..num_monsters {
                let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
                let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

                if !map.is_occupied(x, y) {
                    monsters.push(Monster {
                        kind: monster_choice.ind_sample(&mut rand::thread_rng()),
                        position: Position { x, y },
                    });
                    map.occupied_places.push(Position { x, y });
                }
            }
        }

        monsters
    }

    pub fn build_entities(monsters: Vec<Monster>, world: &mut World) {
        for monster in &monsters {
            match monster.kind {
                MonsterKind::Orc => {
                    world
                        .create_entity()
                        .with(AIControlled)
                        .with(Object {
                            name: String::from("orc"),
                            color: DESATURATED_GREEN,
                            character: 'o',
                        })
                        .with(Health {
                            hp: 20,
                            base_max_hp: 20,
                        })
                        .with(Fighter {
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
                        .with(AIControlled)
                        .with(Object {
                            name: String::from("troll"),
                            color: DARKER_GREEN,
                            character: 'T',
                        })
                        .with(Health {
                            hp: 30,
                            base_max_hp: 30,
                        })
                        .with(Fighter {
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
}
