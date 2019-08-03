use crate::components::{Damage, Fighter};
use specs::{Entities, Join, LazyUpdate, Read, System, WriteStorage};

pub struct TakeDamage;
impl<'a> System<'a> for TakeDamage {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Fighter>,
        WriteStorage<'a, Damage>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entity, mut fighter, mut damages, updater): Self::SystemData) {
        for (entity, fighter, damage) in (&entity, &mut fighter, &mut damages).join() {
            fighter.hp -= damage.base;
            updater.remove::<Damage>(entity);
        }
    }
}
