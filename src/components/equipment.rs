use specs::{Component, Entity, VecStorage};

use crate::components::Equipable;
use crate::item::SlotKind;

pub struct Slot {
    pub kind: SlotKind,
    pub object: Option<Entity>,
}

pub struct Equipment {
    pub slots: Vec<Slot>,
}

impl Component for Equipment {
    type Storage = VecStorage<Self>;
}

impl Equipment {
    pub fn has_equiped(&self, entity: Entity, equipable: &Equipable) -> bool {
        self.get_slot(equipable)
            .map_or(None, |slot| slot.object)
            .map_or(false, |object| object == entity)
    }

    pub fn equip(&mut self, entity: Entity, equipable: &Equipable) {
        self.get_slot_mut(equipable)
            .map(|slot| slot.object = Some(entity));
    }

    pub fn dequip(&mut self, equipable: &Equipable) {
        self.get_slot_mut(equipable).map(|slot| slot.object = None);
    }

    fn get_slot(&self, equipable: &Equipable) -> Option<&Slot> {
        self.slots.iter().find(|slot| slot.kind == equipable.slot)
    }

    fn get_slot_mut(&mut self, equipable: &Equipable) -> Option<&mut Slot> {
        self.slots
            .iter_mut()
            .find(|slot| slot.kind == equipable.slot)
    }
}
