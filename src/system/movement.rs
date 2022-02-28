pub use bevy::prelude::*;

use crate::component;

pub fn movement(mut query: Query<(&component::Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}