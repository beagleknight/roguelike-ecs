pub use tcod::{
    colors,
    input::{self, Event, Key, KeyCode},
};
use tcod::{console::*, Color};

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

#[derive(Clone, PartialEq)]
pub enum Turn {
    Nothing,
    Move,
    PickUp,
    Drop(usize),
    Use(usize),
    Stairs(Option<u32>),
}

pub enum Menu {
    DropItem,
    UseItem,
    LevelUp,
}

pub struct Game {
    pub player_turn: Turn,
    pub key: Key,
    pub menu: Option<Menu>,
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

        Game {
            player_turn: Turn::Nothing,
            key: Default::default(),
            menu: None,
            root,
            log: vec![(
                String::from(
                    "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
                ),
                colors::RED,
            )],
        }
    }

    pub fn read_input(&mut self) {
        self.key = if let Some((_, Event::Key(k))) = input::check_for_event(input::KEY_PRESS) {
            k
        } else {
            Default::default()
        }
    }

    pub fn window_closed(&self) -> bool {
        self.root.window_closed()
    }

    pub fn render_health_bar(&mut self, value: i32, maximum: i32) {
        self.render_bar(
            "HP",
            PANEL_Y + 1,
            value,
            maximum,
            colors::DARKER_RED,
            colors::LIGHT_RED,
        );
    }

    pub fn render_experience_bar(&mut self, value: i32, maximum: i32) {
        self.render_bar(
            "XP",
            PANEL_Y + 2,
            value,
            maximum,
            colors::DARKER_BLUE,
            colors::LIGHT_BLUE,
        );
    }

    fn render_bar(
        &mut self,
        label: &str,
        pos_y: i32,
        value: i32,
        maximum: i32,
        background_color: Color,
        foreground_color: Color,
    ) {
        let bar_width = (value as f32 / maximum as f32 * BAR_WIDTH as f32) as i32;
        self.root.set_default_background(background_color);
        self.root
            .rect(1, pos_y, BAR_WIDTH, 1, false, BackgroundFlag::Screen);
        self.root.set_default_background(foreground_color);
        if bar_width > 0 {
            self.root
                .rect(1, pos_y, bar_width, 1, false, BackgroundFlag::Screen);
        }
        self.root.set_default_background(colors::BLACK);
        self.root.set_default_foreground(colors::WHITE);
        self.root.print_ex(
            1 + BAR_WIDTH / 2,
            pos_y,
            BackgroundFlag::None,
            TextAlignment::Center,
            &format!("{}: {}/{}", label, value, maximum),
        )
    }

    pub fn render_dungeon_level(&mut self, dungeon_level: u32) {
        self.root.set_default_foreground(colors::WHITE);
        self.root.print_ex(
            1,
            PANEL_Y + 3,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("Dungeon level: {}", dungeon_level),
        );
    }

    pub fn render_player_level(&mut self, player_level: u32) {
        self.root.set_default_foreground(colors::WHITE);
        self.root.print_ex(
            1,
            PANEL_Y + 4,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("Player level: {}", player_level),
        );
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

    pub fn clear_window(&mut self) {
        self.root.clear();
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }

    pub fn render_menu<T: AsRef<str>>(&mut self, header: &str, options: &[T], width: i32) {
        assert!(
            options.len() <= 26,
            "Cannot have a menu with more than 26 options."
        );
        let header_height = if header.is_empty() {
            0
        } else {
            self.root
                .get_height_rect(0, 0, width, SCREEN_HEIGHT, header)
        };
        let height = options.len() as i32 + header_height;

        let mut window = Offscreen::new(width, height);
        window.set_default_foreground(colors::WHITE);
        window.print_rect_ex(
            0,
            0,
            width,
            height,
            BackgroundFlag::None,
            TextAlignment::Left,
            header,
        );

        for (index, option_text) in options.iter().enumerate() {
            let menu_letter = (b'a' + index as u8) as char;
            let text = format!("({}) {}", menu_letter, option_text.as_ref());
            window.print_ex(
                0,
                header_height + index as i32,
                BackgroundFlag::None,
                TextAlignment::Left,
                text,
            );
        }

        let x = SCREEN_WIDTH / 2 - width / 2;
        let y = SCREEN_HEIGHT / 2 - height / 2;
        blit(
            &mut window,
            (0, 0),
            (width, height),
            &mut self.root,
            (x, y),
            1.0,
            0.7,
        );

        self.root.flush();
    }
}
