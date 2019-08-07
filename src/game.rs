use tcod::{
    colors::RED,
    console::*,
    input::{self, Event, Key, KeyCode},
    Color,
};

use crate::components::{Object, Position, Tile};
use crate::map::TileKind;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

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

pub enum Turn {
    Nothing,
    Move,
}

pub struct Game {
    pub player_turn: Turn,
    pub key: Key,
    root: Root,
    log: Vec<(String, Color)>,
}

impl Game {
    pub fn create() -> Self {
        let root = Root::initializer()
            .font("terminal8x8_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial [with specs]")
            .init();

        Tcod {
            root,
            key: Default::default(),
            log: vec![(
                String::from(
                    "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
                ),
                RED,
            )],
            player_turn: Turn::Nothing,
        }
    }

    pub fn read_key() -> Key {
        if let Some((_, Event::Key(k))) = input::check_for_event(input::KEY_PRESS) {
            return k;
        }
        Default::default()
    }

    pub fn exit(key: Key) -> bool {
        key.code == KeyCode::Escape
    }

    pub fn render_log(&mut self) {
        let mut y = MSG_HEIGHT as i32;
        for &(ref msg, color) in self.log.iter().rev() {
            let msg_height = self.root.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
            y -= msg_height;
            if y < 0 {
                break;
            }
            self.root.set_default_foreground(color);
            self.root.print_rect(MSG_X, PANEL_Y + y, MSG_WIDTH, 0, msg);
        }
    }

    pub fn log<T: Into<String>>(&mut self, message: T, color: Color) {
        if self.log.len() == MSG_HEIGHT {
            self.log.remove(0);
        }
        self.log.push((message.into(), color));
    }

    pub fn render_object(&mut self, object: &Object, position: &Position, is_in_fov: bool) {
        if is_in_fov {
            self.root.set_default_foreground(object.color);
            self.root.put_char(
                position.x,
                position.y,
                object.character,
                BackgroundFlag::None,
            );
        }
    }

    pub fn render_tile(&mut self, tile: &Tile, position: &Position, is_in_fov: bool) {
        let tile_color = match (tile.kind, tile.explored, is_in_fov) {
            (TileKind::Wall, true, true) => COLOR_LIGHT_WALL,
            (TileKind::Wall, true, false) => COLOR_DARK_WALL,
            (TileKind::Floor, true, true) => COLOR_LIGHT_GROUND,
            (TileKind::Floor, true, false) => COLOR_DARK_GROUND,
            _ => COLOR_DARK_GROUND,
        };

        if tile.explored || is_in_fov {
            self.root
                .set_char_background(position.x, position.y, tile_color, BackgroundFlag::Set);
        }
    }
}
