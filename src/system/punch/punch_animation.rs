use bevy::prelude::*;

use crate::component;

pub fn punch_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::PunchAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut punch_animation, mut transform) in query.iter_mut() {
        punch_animation.duration += time.delta_seconds();
        if punch_animation.duration > 0.2 {
            transform.translation.x = punch_animation.base_x;
            commands
                .entity(entity)
                .remove::<component::PunchAnimation>();
        } else if punch_animation.duration > 0.1 {
            transform.translation.x = punch_animation.base_x
                + (0.2 - punch_animation.duration) * punch_animation.distance;
        } else {
            transform.translation.x =
                punch_animation.base_x + punch_animation.duration * punch_animation.distance;
        }
    }
}
