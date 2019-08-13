use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::*;
use crate::game::{Game, Key, KeyCode, Menu, Turn};

pub struct PlayerAction;
impl<'a> System<'a> for PlayerAction {
    type SystemData = (
        WriteExpect<'a, Game>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Fighter>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut game, mut velocity, mut fighter, mut health, inventory, player): Self::SystemData) {
        game.player_turn = Turn::Nothing;

        for (velocity, inventory, fighter, health, _) in (&mut velocity, &inventory, &mut fighter, &mut health, &player).join() {
            *velocity = Velocity { x: 0, y: 0 };

            if game.menu.is_some() {
                if game.key.code == KeyCode::Char && game.key.printable.is_alphabetic() {
                    let index = game.key.printable.to_ascii_lowercase() as usize - 'a' as usize;
                    let menu = game.menu.take();

                    match menu {
                        Some(Menu::DropItem) if index < inventory.objects.len() => {
                            game.player_turn = Turn::Drop(index);
                        }
                        Some(Menu::UseItem) if index < inventory.objects.len() => {
                            game.player_turn = Turn::Use(index);
                        }
                        Some(Menu::LevelUp) => match index {
                            0 => {
                                health.base_max_hp += 20;
                                health.hp += 20;
                            }
                            1 => {
                                fighter.base_power += 1;
                            }
                            2 => {
                                fighter.base_defense += 1;
                            }
                            _ => game.menu = Some(Menu::LevelUp),
                        },
                        _ => {}
                    }
                }
            } else {
                match game.key {
                    Key {
                        code: KeyCode::Up, ..
                    } => {
                        *velocity = Velocity { x: 0, y: -1 };
                        game.player_turn = Turn::Move;
                    }
                    Key {
                        code: KeyCode::Down,
                        ..
                    } => {
                        *velocity = Velocity { x: 0, y: 1 };
                        game.player_turn = Turn::Move;
                    }
                    Key {
                        code: KeyCode::Left,
                        ..
                    } => {
                        *velocity = Velocity { x: -1, y: 0 };
                        game.player_turn = Turn::Move;
                    }
                    Key {
                        code: KeyCode::Right,
                        ..
                    } => {
                        *velocity = Velocity { x: 1, y: 0 };
                        game.player_turn = Turn::Move;
                    }
                    Key {
                        code: KeyCode::Char,
                        printable: 'g',
                        ..
                    } => {
                        game.player_turn = Turn::PickUp;
                    }
                    Key {
                        code: KeyCode::Char,
                        printable: 'd',
                        ..
                    } => {
                        game.menu = Some(Menu::DropItem);
                    }
                    Key {
                        code: KeyCode::Char,
                        printable: 'i',
                        ..
                    } => {
                        game.menu = Some(Menu::UseItem);
                    }
                    Key {
                        code: KeyCode::Char,
                        printable: 's',
                        ..
                    } => {
                        game.player_turn = Turn::Stairs(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
