use crate::components::{AIControlled, Player, Position, Velocity};
use crate::tcod::Tcod;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct AIVelocity;
impl<'a> System<'a> for AIVelocity {
    type SystemData = (
        ReadExpect<'a, Tcod>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, AIControlled>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (_tcod, mut velocity, ai_controlled, position, player): Self::SystemData) {
        let player_position: Position = (&position, &player)
            .join()
            .map({ |(position, _)| position.clone() })
            .nth(0)
            .unwrap();

        for (velocity, position, _) in (&mut velocity, &position, &ai_controlled).join() {
            // TODO: check fov
            let dx = player_position.x - position.x;
            let dy = player_position.y - position.y;
            let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

            let dx = (dx as f32 / distance).round() as i32;
            let dy = (dy as f32 / distance).round() as i32;

            *velocity = Velocity { x: dx, y: dy };
        }
    }
}
