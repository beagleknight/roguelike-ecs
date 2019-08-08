use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Inventory, Player, Velocity};
use crate::game::{Game, Key, KeyCode, Turn};

pub struct PlayerAction;
impl<'a> System<'a> for PlayerAction {
    type SystemData = (
        WriteExpect<'a, Game>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut game, mut velocity, inventory, player): Self::SystemData) {
        for (velocity, inventory, _) in (&mut velocity, &inventory, &player).join() {
            if game.inventory_opened {
                if game.key.code == KeyCode::Char && game.key.printable.is_alphabetic() {
                    let index = game.key.printable.to_ascii_lowercase() as usize - 'a' as usize;
                    game.inventory_opened = false;
                    if index < inventory.objects.len() {
                        game.player_turn = Turn::Drop(index);
                    } else {
                        game.player_turn = Turn::Nothing;
                    }
                } else {
                    game.player_turn = Turn::Nothing;
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
                        game.inventory_opened = true;
                        game.player_turn = Turn::Nothing;
                    }
                    _ => {
                        *velocity = Velocity { x: 0, y: 0 };
                        game.player_turn = Turn::Nothing;
                    }
                }
            }
        }
    }
}
