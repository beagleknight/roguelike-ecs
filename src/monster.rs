use crate::components::{Position};
use rand::{
    distributions::{IndependentSample, Weighted, WeightedChoice},
    Rng,
};

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
                        position: Position { x, y }
                    });
                    map.occupied_places.push(Position { x, y });
                }
            }
        }

        monsters
    }
}
