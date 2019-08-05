use tcod::{
    colors::RED,
    console::*,
    input::{self, Event, Key, KeyCode},
    Color,
};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

pub struct Tcod {
    pub root: Root,
    pub key: Key,
    log: Vec<(String, Color)>,
}

impl Tcod {
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
}
