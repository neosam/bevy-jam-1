use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod component;
mod resource;
mod shapes;
mod system;


pub struct ScoredEvent {
    pub side: component::Side,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InGame,
    Won,
    Lost,
}


fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Such an unfair Multiball Pong!  My paddle is too small!!!1".to_string(),
            width: 1000.0,
            height: 600.0,
            ..Default::default()
        })
        .add_state(GameState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_event::<ScoredEvent>()
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(system::setup))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(system::user_input)
                .with_system(system::movement)
                .with_system(system::ball_bounds_check)
                .with_system(system::paddle_collision)
                .with_system(system::score)
                .with_system(system::keep_paddle_in_screen)
                .with_system(system::ai),
        )
        .add_system_set(SystemSet::on_enter(GameState::Won).with_system(system::setup_won))
        .add_system_set(SystemSet::on_enter(GameState::Lost).with_system(system::setup_lost))
        .run();
}
