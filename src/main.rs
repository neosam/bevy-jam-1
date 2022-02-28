use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

mod component;
mod resource;
mod shapes;
mod system;

const X_BOUNDS_LEFT: f32 = -500.0;
const X_BOUNDS_RIGHT: f32 = 500.0;
const Y_BOUNDS_TOP: f32 = 300.0;
const Y_BOUNDS_BOTTOM: f32 = -300.0;
const PEDAL_ACCELERATION: f32 = 1500.0;
const PEDAL_BREAK: f32 = 900.0;
const PEDAL_MAX_SPEED: f32 = 400.0;
const BOUNCE_ACCELERATION: f32 = 1.1;
const INIT_VELOCITY_X: f32 = 120.0;
const INIT_VELOCITY_Y: f32 = 90.0;
const MAX_BALLS: usize = 4;


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
