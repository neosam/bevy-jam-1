use bevy::prelude::*;

use crate::component;

pub fn punch_action(
    mut ball_query: Query<(&mut component::Velocity, &Transform), With<component::Ball>>,
    paddle_query: Query<
        (&Transform, &component::Side, &component::Punch),
        (With<component::Paddle>, Without<component::ColliderOnly>),
    >,
    mut punch_event_reader: EventReader<crate::StartPunchEvent>,
) {
    for event in punch_event_reader.iter() {
        for (paddle_transform, _, punch) in paddle_query
            .iter()
            .filter(|(_, side, punch)| event.side == **side && punch.strength >= 1.0)
        {
            for (mut velocity, ball_transform) in
                ball_query.iter_mut().filter(|(_, ball_transform)| {
                    (paddle_transform.translation - ball_transform.translation).length()
                        <= punch.strength * 200.0
                })
            {
                let mut distance = ball_transform.translation - paddle_transform.translation;
                if distance.y > 0. && distance.y > distance.x {
                    distance.y = distance.x;
                } else if distance.y < 0. && distance.y < -distance.x {
                    distance.y = -distance.x;
                }
                let new_velocity = distance.normalize() * punch.strength * 300.0;
                velocity.velocity.x = new_velocity.x;
                velocity.velocity.y = new_velocity.y;
            }
        }
    }
}
