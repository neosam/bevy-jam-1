use bevy::prelude::*;
use crate::component;
use crate::resource;

pub fn ball_bounds_check(
    mut query: Query<(&mut component::Velocity, &mut Transform), With<component::Ball>>,
    mut score_event_writer: EventWriter<crate::ScoredEvent>,
    game_globals: Res<resource::GameGlobals>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        if transform.translation.y < game_globals.bounds_bottom {
            transform.translation.y = game_globals.bounds_bottom;
            if velocity.velocity.y < 0.0 {
                velocity.velocity.y *= -1.0;
            }
        }
        if transform.translation.y > game_globals.bounds_top {
            transform.translation.y = game_globals.bounds_top;
            if velocity.velocity.y > 0.0 {
                velocity.velocity.y *= -1.0;
            }
        }
        if transform.translation.x < game_globals.bounds_left {
            transform.translation.x = 0.0;
            velocity.velocity = game_globals.ball_init_velocity;
            score_event_writer.send(crate::ScoredEvent { side: component::Side::Right });
        }
        if transform.translation.x > game_globals.bounds_right {
            transform.translation.x = 0.0;
            velocity.velocity = game_globals.ball_init_velocity;
            score_event_writer.send(crate::ScoredEvent { side: component::Side::Left });
        }
    }
}

pub fn paddle_collision(
    mut ball_query: Query<(&mut component::Velocity, &GlobalTransform, &component::Collider), With<component::Ball>>,
    pedal_query: Query<(&GlobalTransform, &component::Side, &component::Collider), With<component::Paddle>>,
    game_globals: Res<resource::GameGlobals>,
    mut paddle_collision_events: EventWriter<crate::PaddleCollisionEvent>,
) {
    for (mut ball_velocity, ball_transform, ball_collider) in ball_query.iter_mut() {
        let ball_left = ball_collider.left(ball_transform);
        let ball_right = ball_collider.right(ball_transform);
        let ball_top = ball_collider.top(ball_transform);
        let ball_bottom = ball_collider.bottom(ball_transform);
        for (pedal_transform, pedal_side, pedal_collider) in pedal_query.iter() {
            let combined_colliders = pedal_collider.add(ball_collider);
            let most_left = ball_left.min(pedal_collider.left(pedal_transform));
            let most_right = ball_right.max(pedal_collider.right(pedal_transform));
            let most_top = ball_top.max(pedal_collider.top(pedal_transform));
            let most_bottom = ball_bottom.min(pedal_collider.bottom(pedal_transform));
            if combined_colliders.dimension.x * 2.0 > most_right - most_left
                && combined_colliders.dimension.y * 2.0 > most_top - most_bottom
            {
                let reflection_angle = (pedal_transform.translation.y
                    - ball_transform.translation.y)
                    / combined_colliders.dimension.y
                    * std::f32::consts::PI
                    / 4.0;
                let new_velocity_x =
                    reflection_angle.cos() * ball_velocity.velocity.length() * game_globals.speed_increase_per_bounce;
                let new_velocity_y =
                    -reflection_angle.sin() * ball_velocity.velocity.length() * game_globals.speed_increase_per_bounce;

                match *pedal_side {
                    component::Side::Left => {
                        if ball_velocity.velocity.x < 0.0 {
                            ball_velocity.velocity.x = new_velocity_x;
                            ball_velocity.velocity.y = new_velocity_y;
                            paddle_collision_events.send(crate::PaddleCollisionEvent { side: *pedal_side });
                        }
                    }
                    component::Side::Right => {
                        if ball_velocity.velocity.x > 0.0 {
                            ball_velocity.velocity.x = -new_velocity_x;
                            ball_velocity.velocity.y = new_velocity_y;
                            paddle_collision_events.send(crate::PaddleCollisionEvent { side: *pedal_side });
                        }
                    }
                }
            }
        }
    }
}

pub fn keep_paddle_in_screen(
    mut query: Query<(&mut Transform, &mut component::Velocity, &component::Collider), With<component::Paddle>>,
) {
    for (mut transform, mut velocity, collider) in query.iter_mut() {
        let upper_border = 300.0 - collider.dimension.y;
        let lower_border = -300.0 + collider.dimension.y;

        let y_buffer = transform.translation.y;
        transform.translation.y = transform.translation.y.max(lower_border).min(upper_border);
        if y_buffer != transform.translation.y {
            velocity.velocity.y = 0.0;
        }
    }
}