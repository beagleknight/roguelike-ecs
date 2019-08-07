use rand::Rng;
use specs::prelude::*;
use std::cmp;
use tcod::{
    colors::Color,
    map::{FovAlgorithm, Map as Fov},
};

use crate::components::renderable::Arrangement;
use crate::components::{Block, Position, Renderable};

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

pub type FovMap = Vec<Vec<TileVisibility>>;

#[derive(Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub enum TileVisibility {
    Visible,
    NotVisible,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
    pub player_starting_position: Position,
    pub occupied_places: Vec<Position>,
    pub fov: Fov,
}

impl Map {
    pub fn create() -> Map {
        let mut map = Self::empty();

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

            let new_room = Room::new(x, y, w, h);
            let failed = map
                .rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                map.create_room(new_room);
                let (new_x, new_y) = new_room.center();
                if map.rooms.is_empty() {
                    map.player_starting_position = Position { x: new_x, y: new_y };
                } else {
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rand::random() {
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        map.occupied_places
            .push(map.player_starting_position.clone());

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                map.fov.set(
                    x,
                    y,
                    map.tiles[x as usize][y as usize] == Tile::Floor,
                    map.tiles[x as usize][y as usize] == Tile::Floor,
                );
            }
        }

        map
    }

    pub fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.occupied_places
            .iter()
            .any(|position| position.x == x && position.y == y)
    }

    fn empty() -> Self {
        Map {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            tiles: vec![vec![Tile::Wall; MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            rooms: vec![],
            player_starting_position: Position { x: 0, y: 0 },
            occupied_places: vec![],
            fov: Fov::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }

    fn create_room(&mut self, room: Room) {
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

    pub fn build_entities(&self, world: &mut World) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles[x as usize][y as usize] {
                    Tile::Wall => {
                        world
                            .create_entity()
                            .with(Renderable {
                                color: COLOR_LIGHT_WALL,
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
                                color: COLOR_LIGHT_GROUND,
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
    }

    pub fn recompute_fov(&mut self, player_position: &Position) -> FovMap {
        let mut fov_map =
            vec![vec![TileVisibility::NotVisible; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        self.fov.compute_fov(
            player_position.x,
            player_position.y,
            TORCH_RADIUS,
            FOV_LIGHT_WALLS,
            FOV_ALGO,
        );

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                fov_map[x as usize][y as usize] = if self.fov.is_in_fov(x, y) {
                    TileVisibility::Visible
                } else {
                    TileVisibility::NotVisible
                };
            }
        }

        fov_map
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Room {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Room {
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

    pub fn intersects_with(&self, other: &Room) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}
