use specs::{System, Write};
use tcod::input::Key;
use tcod::input::{self, Event};

pub struct ReadKeyboard;
impl<'a> System<'a> for ReadKeyboard {
    type SystemData = Write<'a, Key>;

    fn run(&mut self, mut key: Self::SystemData) {
        match input::check_for_event(input::KEY_PRESS) {
            Some((_, Event::Key(k))) => {
                println!("key!");
                *key = k;
            }
            _ => {
                *key = Default::default();
            }
        }
    }
}
