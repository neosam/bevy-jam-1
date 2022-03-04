pub use bevy::prelude::*;

use crate::component;
use crate::resource;

pub fn movement(
    mut query: Query<(
        &mut component::Velocity,
        &mut Transform,
        Option<&component::Damping>,
    )>,
    time: Res<Time>,
    game_globals: Res<resource::GameGlobals>,
) {
    for (mut velocity, mut transform, damping) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
        if damping.is_some() {
            if velocity.velocity.y > 0.0 {
                velocity.velocity.y = 0.0f32
                    .max(velocity.velocity.y - game_globals.paddle_break * time.delta_seconds());
            } else if velocity.velocity.y < 0.0 {
                velocity.velocity.y = 0.0f32
                    .min(velocity.velocity.y + game_globals.paddle_break * time.delta_seconds());
            }
        }
    }
}
