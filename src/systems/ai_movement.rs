use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{Block, Player, Position, Velocity};

pub struct AIMovement;
impl<'a> System<'a> for AIMovement {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut position, velocity, block, player): Self::SystemData) {
        let mut occupied_positions: Vec<Position> = (&mut position, &block)
            .join()
            .map({ |(position, _)| position.clone() })
            .collect();

        for (position, velocity, _) in (&mut position, &velocity, !&player).join() {
            let blocked = occupied_positions
                .iter()
                .any(|occupied_position| *occupied_position == position.clone() + velocity.clone());

            if !blocked {
                *position = position.clone() + velocity.clone();
                occupied_positions.push(position.clone());
            }
        }
    }
}
