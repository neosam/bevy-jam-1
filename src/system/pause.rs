use bevy::prelude::*;

pub fn pause(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<crate::GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        bevy::log::info!("Unpause the game");
        keyboard.clear();
        if let Err(err) = state.pop() {
            bevy::log::error!("Could not unpause the game: {err}");
        }
    }
}
