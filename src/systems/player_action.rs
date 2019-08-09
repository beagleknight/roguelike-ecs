use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Inventory, Player, Velocity};
use crate::game::{Game, Key, KeyCode, Turn, InventoryAction};

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
            if game.inventory_action.is_some() {
                if game.key.code == KeyCode::Char && game.key.printable.is_alphabetic() {
                    let index = game.key.printable.to_ascii_lowercase() as usize - 'a' as usize;
                    let inventory_action = game.inventory_action.take();
                    if index < inventory.objects.len() {
                        match inventory_action {
                            Some(InventoryAction::Drop) =>  game.player_turn = Turn::Drop(index),
                            Some(InventoryAction::Use) =>  game.player_turn = Turn::Use(index),
                            None => unreachable!()
                        }
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
                        game.inventory_action = Some(InventoryAction::Drop);
                        game.player_turn = Turn::Nothing;
                    }
                    Key {
                        code: KeyCode::Char,
                        printable: 'i',
                        ..
                    } => {
                        game.inventory_action = Some(InventoryAction::Use);
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
