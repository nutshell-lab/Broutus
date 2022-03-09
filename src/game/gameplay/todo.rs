use super::MapPosition;
use bevy::prelude::*;

// Add a property skills to the existing Warrior struct
pub struct Warrior {
    name: Name,
    skills: [Skill],
}

// A skill would be a composable data-struct
pub struct Skill {
    name: Name,
    handling_type: SkillHandling,
    effects: [SkillEffect],
}

// An enum would allow the interface to maximize user-friendlyness depending on the Skill
pub enum SkillHandling {
    SingleTarget,
    MultipleTargets(i32), // count
    Aoe(i32),             //radius
}

// A Skill could have several SkillEffects. SkillEffects would be emitted as events
pub enum SkillEffect {
    Healing(MapPosition, u32),
    Dammage(MapPosition, u32),
}

// A system will set the selected_skill in the phase and set it as a resource. This system would emits the effects-events within the skill.
pub fn activate_skill_system(selected_skill: Res<Skill>, mut ev_writter: EventWriter<SkillEffect>) {
    for effect in selected_skill.effects {
        ev_writter.send(effect)
    }
}

// Poorly coded sample of a system reacting to a SkillEffect event.
pub fn healing_system(
    mut healing_event: EventReader<HealTarget>,
    target_query: Query<(&mut Health, MapPosition)>,
) {
    for ev in healing_event.iter() {
        let (map_position, amount) = ev;
        let (health, _) = target_query.get(map_position);
    }
}
