use tcod::console::*;
use tcod::input::{self, Event, Key, KeyCode};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

pub struct Tcod {
    pub root: Root,
    pub key: Key,
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
}
