use bevy::prelude::*;

use crate::component;

pub fn start_punch(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut component::Punch, &Transform, &component::Side),
        (
            With<component::Paddle>,
            Without<component::ColliderOnly>,
            Without<component::PunchAnimation>,
        ),
    >,
    mut start_punch_events: EventReader<crate::StartPunchEvent>,
) {
    for event in start_punch_events.iter() {
        for (entity, mut punch, transform, _) in query
            .iter_mut()
            .filter(|(_, punch, _, side)| **side == event.side && punch.strength >= 1.0)
        {
            commands.entity(entity).insert(component::PunchAnimation {
                duration: 0.0,
                distance: punch.strength * 50.0,
                base_x: transform.translation.x,
            });
            punch.strength = 0.0;
        }
    }
}
