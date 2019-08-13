use specs::{Join, WriteExpect, ReadStorage, System, WriteStorage};

use crate::components::*;
use crate::game::{Menu, Game, colors};
use crate::player::{LEVEL_UP_BASE, LEVEL_UP_FACTOR};

pub struct PlayerLevelUp;

impl<'a> System<'a> for PlayerLevelUp {
    type SystemData = (WriteExpect<'a, Game>, WriteStorage<'a, Experience>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut game, mut experiences, players) = data;

        for (experience, _) in (&mut experiences, &players).join() {
            if let Some(next_level_points) = experience.next_level_points {
                if experience.points >= next_level_points {
                    experience.level += 1;
                    experience.next_level_points = Some(LEVEL_UP_BASE + experience.level * LEVEL_UP_FACTOR);
                    experience.points = 0;
                    game.log(
                        format!(
                            "Your battle skills grow stronger! You reached level {}!",
                            experience.level
                        ),
                        colors::YELLOW,
                    );
                    game.menu = Some(Menu::LevelUp);
                }
            }
        }
    }
}
