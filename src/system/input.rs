use bevy::prelude::*;
use crate::component;

pub fn user_input(
    mut query: Query<&mut component::Velocity, With<component::Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard.pressed(KeyCode::W) {
            velocity.velocity.y = crate::PEDAL_MAX_SPEED
                .min(velocity.velocity.y + crate::PEDAL_ACCELERATION * time.delta_seconds());
        }
        if keyboard.pressed(KeyCode::S) {
            velocity.velocity.y = (-crate::PEDAL_MAX_SPEED)
                .max(velocity.velocity.y - crate::PEDAL_ACCELERATION * time.delta_seconds());
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