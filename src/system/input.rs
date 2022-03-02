use bevy::prelude::*;
use crate::component;
use crate::resource;

pub fn user_input(
    mut query: Query<&mut component::Velocity, With<component::Player>>,
    mut keyboard: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    game_globals: Res<resource::GameGlobals>,
    mut state: ResMut<State<crate::GameState>>,
    mut start_punch_event_writer: EventWriter<crate::StartPunchEvent>,
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
        if keyboard.just_pressed(KeyCode::D) {
            start_punch_event_writer.send(crate::StartPunchEvent {side: component::Side::Left});
        }
        if keyboard.just_pressed(KeyCode::Space) {
            bevy::log::info!("Switch to pause state");
            keyboard.clear();
            if let Err(err) = state.push(crate::GameState::Pause) {
                bevy::log::error!("Could not pause the game: {}", err);
            }
        }
    }
}