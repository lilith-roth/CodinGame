use crate::{
    CHARACTER_KILL_RANGE,
    CHARACTER_MOVE_SPEED,
    math,
    ZOMBIE_KILL_RANGE,
    ZOMBIE_MOVE_SPEED
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Position(pub(crate) i32, pub(crate) i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Entity {
    pub(crate) id: i32,
    pub(crate) position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Human {
    pub(crate) entity: Entity,
    pub(crate) zombies_facing_human: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Zombie {
    pub(crate) entity: Entity,
    pub(crate) position_next: Position,
}

impl Zombie {
    pub(crate) fn can_character_save_in_time(self, character_position: Position, target_position: Position) -> bool {
        let target_distance = math::calculate_distance(self.entity.position, target_position);
        let character_distance = math::calculate_distance(character_position, target_position);
        let a = target_distance / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED);
        let b = character_distance / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED);
        let can_be_saved = target_distance / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED) > character_distance / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED);
        eprintln!("({}, {}): a {} | b {} | {}",
                  target_position.0,
                  target_position.1,
                  a,
                  b,
                  can_be_saved
        );
        can_be_saved
    }

    pub(crate) fn is_zombie_targeting_entity(&self, target_entity_position: Position) -> bool {
        math::calculate_distance(target_entity_position, self.position_next)
            + math::calculate_distance(self.entity.position, self.position_next)
            == math::calculate_distance(target_entity_position, self.entity.position)
    }
}
