use crate::components::{AIControlled, Block, Player, Position, Velocity};
use crate::tcod::{Tcod, Turn};
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct AIVelocity;
impl<'a> System<'a> for AIVelocity {
    type SystemData = (
        ReadExpect<'a, Tcod>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, AIControlled>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Block>,
    );

    fn run(
        &mut self,
        (tcod, mut velocity, ai_controlled, position, player, block): Self::SystemData,
    ) {
        match tcod.player_turn {
            Turn::Nothing => {
                for (velocity, _, _) in (&mut velocity, &position, &ai_controlled).join() {
                    *velocity = Velocity { x: 0, y: 0 };
                }
            }
            _ => {
                let occupied_positions: Vec<Position> = (&position, &block, !&player)
                    .join()
                    .map({ |(position, _, _)| position.clone() })
                    .collect();
                let player_position: Position = (&position, &player)
                    .join()
                    .map({ |(position, _)| position.clone() })
                    .nth(0)
                    .unwrap();

                for (velocity, position, _) in (&mut velocity, &position, &ai_controlled).join() {
                    // TODO: add fov
                    if position.distance_to(&player_position) > 5.0 {
                        *velocity = Velocity { x: 0, y: 0 };
                        continue;
                    }

                    let mut best_distance = Some(player_position.distance_to(&position));
                    let mut best_velocity = Some(Velocity { x: 0, y: 0 });

                    for x in -1..=1 {
                        for y in -1..=1 {
                            if x == 0 && y == 0 {
                                continue;
                            }

                            let next_position = Position {
                                x: position.x + x,
                                y: position.y + y,
                            };
                            let blocked = occupied_positions.iter().any(|occupied_position| {
                                occupied_position.x == next_position.x
                                    && occupied_position.y == next_position.y
                            });

                            if !blocked {
                                let distance = player_position.distance_to(&next_position);
                                match best_distance {
                                    Some(best) => {
                                        if distance < best {
                                            best_distance = Some(distance);
                                            best_velocity =
                                                Some(position.direction_to(&next_position));
                                        }
                                    }
                                    None => {
                                        best_distance = Some(distance);
                                        best_velocity = Some(position.direction_to(&next_position));
                                    }
                                }
                            }
                        }
                    }

                    if let Some(best_velocity) = best_velocity {
                        *velocity = best_velocity;
                    }
                }
            }
        }
    }
}