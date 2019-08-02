use rand::{
    distributions::{IndependentSample, Weighted, WeightedChoice},
    Rng,
};
use specs::prelude::*;
use std::cmp;
use tcod::colors::Color;
use tcod::colors::DARKER_GREEN;
use tcod::colors::DESATURATED_GREEN;

use crate::components::renderable::Arrangement;
use crate::components::{Block, Position, Renderable};

const MAX_MONSTERS: i32 = 3;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

#[derive(Clone)]
enum Tile {
    Wall,
    Floor,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn empty() -> Self {
        let tiles = vec![vec![Tile::Wall; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        Map { tiles }
    }

    pub fn create(world: &mut World) -> Position {
        let mut map = Self::empty();
        let mut rooms = vec![];
        let mut starting_position = Position { x: 0, y: 0 };

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);
            let failed = rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                map.create_room(new_room);
                let (new_x, new_y) = new_room.center();
                if rooms.is_empty() {
                    starting_position = Position { x: new_x, y: new_y };
                } else {
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if rand::random() {
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }
                rooms.push(new_room);
                map.place_objects(new_room, world);
            }
        }

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                match map.tiles[x as usize][y as usize] {
                    Tile::Wall => {
                        world
                            .create_entity()
                            .with(Renderable {
                                color: COLOR_DARK_WALL,
                                character: None,
                                arrangement: Arrangement::Background,
                            })
                            .with(Position {
                                x: x as i32,
                                y: y as i32,
                            })
                            .with(Block)
                            .build();
                    }
                    Tile::Floor => {
                        world
                            .create_entity()
                            .with(Renderable {
                                color: COLOR_DARK_GROUND,
                                character: None,
                                arrangement: Arrangement::Background,
                            })
                            .with(Position {
                                x: x as i32,
                                y: y as i32,
                            })
                            .build();
                    }
                }
            }
        }

        starting_position
    }

    fn create_room(&mut self, room: Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.tiles[x as usize][y as usize] = Tile::Floor;
            }
        }
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::Floor;
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::Floor;
        }
    }

    fn place_objects(&self, room: Rect, world: &mut World) {
        let num_monsters = rand::thread_rng().gen_range(0, MAX_MONSTERS + 1);
        let monster_chances = &mut [
            Weighted {
                weight: 80,
                item: "orc",
            },
            Weighted {
                weight: 20,
                item: "troll",
            },
        ];
        let monster_choice = WeightedChoice::new(monster_chances);

        for _ in 0..num_monsters {
            let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

            if true {
                // !is_blocked(x, y, map, objects) {
                match monster_choice.ind_sample(&mut rand::thread_rng()) {
                    "orc" => {
                        world
                            .create_entity()
                            .with(Renderable {
                                color: DESATURATED_GREEN,
                                character: Some('o'),
                                arrangement: Arrangement::Foreground,
                            })
                            .with(Position { x, y })
                            .with(Block)
                            .build();
                        // let mut orc = Object::new(x, y, 'o', "orc", DESATURATED_GREEN, true);
                        // orc.fighter = Some(Fighter {
                        //     hp: 20,
                        //     base_max_hp: 20,
                        //     base_defense: 0,
                        //     base_power: 4,
                        //     on_death: DeathCallback::Monster,
                        //     xp: 35,
                        // });
                        // orc.ai = Some(Ai::Basic);
                        // orc
                    }
                    "troll" => {
                        world
                            .create_entity()
                            .with(Renderable {
                                color: DARKER_GREEN,
                                character: Some('T'),
                                arrangement: Arrangement::Foreground,
                            })
                            .with(Position { x, y })
                            .with(Block)
                            .build();
                        // let mut troll = Object::new(x, y, 'T', "troll", DARKER_GREEN, true);
                        // troll.fighter = Some(Fighter {
                        //     hp: 30,
                        //     base_max_hp: 30,
                        //     base_defense: 2,
                        //     base_power: 8,
                        //     on_death: DeathCallback::Monster,
                        //     xp: 100,
                        // });
                        // troll.ai = Some(Ai::Basic);
                        // troll
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}
