use bevy::prelude::*;
use crate::component;

pub fn ai(
    mut ai_query: Query<(&mut component::Velocity, &Transform), With<component::AI>>,
    ball_query: Query<&Transform, With<component::Ball>>,
    time: Res<Time>,
) {
    for (mut velocity, transform) in ai_query.iter_mut() {
        let most_relevant_ball = ball_query
            .iter()
            .map(|ball_transform| (ball_transform.translation.x, ball_transform.translation.y))
            .fold(None, |acc, (x, y)| {
                if let Some((acc_x, acc_y)) = acc {
                    if x > acc_x {
                        Some((x, y))
                    } else {
                        Some((acc_x, acc_y))
                    }
                } else {
                    Some((x, y))
                }
            });
        if let Some((_, ball_y)) = most_relevant_ball {
            if ball_y < transform.translation.y {
                velocity.velocity.y = (-crate::PEDAL_MAX_SPEED)
                    .max(velocity.velocity.y - crate::PEDAL_ACCELERATION * time.delta_seconds());
            } else {
                velocity.velocity.y = crate::PEDAL_MAX_SPEED
                    .min(velocity.velocity.y + crate::PEDAL_ACCELERATION * time.delta_seconds());
            }
            if velocity.velocity.y > 0.0 {
                velocity.velocity.y =
                    0.0f32.max(velocity.velocity.y - crate::PEDAL_BREAK * time.delta_seconds());
            } else if velocity.velocity.y < 0.0 {
                velocity.velocity.y =
                    0.0f32.min(velocity.velocity.y + crate::PEDAL_BREAK * time.delta_seconds());
            }
        }
    }
}