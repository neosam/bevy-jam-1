use bevy::prelude::*;
use crate::component;

pub fn punch_increase(
    mut query: Query<(&mut component::Punch, &component::Side)>,
    mut events: EventReader<crate::PaddleCollisionEvent>,
) {
    for event in events.iter() {
        for (mut punch, side) in query.iter_mut() {
            if event.side == *side {
                punch.strength = (punch.strength + 0.2).min(punch.max_strength);
            }
        }
    }
}