use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{Player, Velocity};
use crate::game::{Game, Key, KeyCode, Turn};

pub struct PlayerVelocity;
impl<'a> System<'a> for PlayerVelocity {
    type SystemData = (
        WriteExpect<'a, Game>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut game, mut velocity, player): Self::SystemData) {
        for (velocity, _) in (&mut velocity, &player).join() {
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
                _ => {
                    *velocity = Velocity { x: 0, y: 0 };
                    game.player_turn = Turn::Nothing;
                }
            }
        }
    }
}
