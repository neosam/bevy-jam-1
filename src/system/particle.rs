use crate::component;
use crate::particles;
use bevy::prelude::*;

pub fn ball_bounce_particles(
    mut commands: Commands,
    query: Query<&Transform>,
    mut paddle_collision_events: EventReader<crate::PaddleCollisionEvent>,
    time: Res<Time>,
) {
    for event in paddle_collision_events.iter() {
        if let Ok(transform) = query.get_component::<Transform>(event.ball_entity) {
            particles::Particles {
                amount: 200,
                min_speed: 3.0,
                max_speed: 100.0,
                color: Color::rgb(0.37, 0.07, 0.07),
                min_time_to_live: 1.0,
                max_time_to_live: 2.0,
            }
            .generate(
                &mut commands,
                particles::Random::new(time.time_since_startup().as_secs()),
                *transform,
            )
        }
    }
}

pub fn punch_particles(
    mut commands: Commands,
    query: Query<(&Transform, &component::Punch, &component::Side)>,
    mut punch_events: EventReader<crate::StartPunchEvent>,
    time: Res<Time>,
) {
    for event in punch_events.iter() {
        for (transform, punch) in query
            .iter()
            .filter(|(_, _, side)| **side == event.side)
            .map(|(transform, punch, _)| (transform, punch))
        {
            particles::Particles {
                amount: (punch.strength * 200.0) as u32,
                min_speed: punch.strength * 90.0,
                max_speed: punch.strength * 100.0,
                color: Color::RED,
                min_time_to_live: 1.0,
                max_time_to_live: 2.0,
            }
            .generate(
                &mut commands,
                particles::Random::new(time.time_since_startup().as_secs()),
                *transform,
            )
        }
    }
}

pub fn time_to_live(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::TimeToLive)>,
    time: Res<Time>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.ttl -= time.delta_seconds();
        if ttl.ttl < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
