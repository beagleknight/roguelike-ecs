use crate::components::{Player, Velocity};
use crate::tcod::{Tcod, Turn};
use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::input::{
    Key,
    KeyCode::{Down, Left, Right, Up},
};

pub struct PlayerVelocity;
impl<'a> System<'a> for PlayerVelocity {
    type SystemData = (
        WriteExpect<'a, Tcod>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut tcod, mut velocity, player): Self::SystemData) {
        for (velocity, _) in (&mut velocity, &player).join() {
            match tcod.key {
                Key { code: Up, .. } => {
                    *velocity = Velocity { x: 0, y: -1 };
                    tcod.player_turn = Turn::Move;
                }
                Key { code: Down, .. } => {
                    *velocity = Velocity { x: 0, y: 1 };
                    tcod.player_turn = Turn::Move;
                }
                Key { code: Left, .. } => {
                    *velocity = Velocity { x: -1, y: 0 };
                    tcod.player_turn = Turn::Move;
                }
                Key { code: Right, .. } => {
                    *velocity = Velocity { x: 1, y: 0 };
                    tcod.player_turn = Turn::Move;
                }
                _ => {
                    *velocity = Velocity { x: 0, y: 0 };
                    tcod.player_turn = Turn::Nothing;
                }
            }
        }
    }
}
