use super::MapPosition;
use bevy::prelude::*;

pub struct Warrior {
    name: Name,
    skills: [Skill],
}

pub struct Skill {
    name: Name,
    handling_type: SkillHandling,
    effects: [SkillEffect],
}

pub enum SkillHandling {
    SingleTarget,
    MultipleTargets(i32), // count
    Aoe(i32),             //radius
}
pub enum SkillEffect {
    Healing(MapPosition, u32),
    Dammage(MapPosition, u32),
}

// A system will set the selected_skill in the phase and set it as a rest
pub fn activate_skill_system(selected_skill: Res<Skill>, mut ev_writter: EventWriter<SkillEffect>) {
    for effect in selected_skill.effects {
        ev_writter.send(effect)
    }
}

pub fn healing_system(
    mut healing_event: EventReader<HealTarget>,
    target_query: Query<(&mut Health, MapPosition)>,
) {
    for ev in healing_event.iter() {
        let (map_position, amount) = ev;
        let (health, _) = target_query.get(map_position);
    }
}
