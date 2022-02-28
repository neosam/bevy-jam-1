use bevy::prelude::*;
use crate::component;
use crate::resource;

pub fn user_input(
    mut query: Query<&mut component::Velocity, With<component::Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    game_globals: Res<resource::GameGlobals>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard.pressed(KeyCode::W) {
            velocity.velocity.y = game_globals.paddle_max_speed 
                .min(velocity.velocity.y + game_globals.paddle_acceleration * time.delta_seconds());
        }
        if keyboard.pressed(KeyCode::S) {
            velocity.velocity.y = (-game_globals.paddle_max_speed)
                .max(velocity.velocity.y - game_globals.paddle_acceleration * time.delta_seconds());
        }
        if velocity.velocity.y > 0.0 {
            velocity.velocity.y =
                0.0f32.max(velocity.velocity.y - game_globals.paddle_break * time.delta_seconds());
        } else if velocity.velocity.y < 0.0 {
            velocity.velocity.y =
                0.0f32.min(velocity.velocity.y + game_globals.paddle_break * time.delta_seconds());
        }
    }
}