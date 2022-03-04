use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::RegisterInspectable;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;

mod bundle;
mod component;
mod particles;
mod resource;
mod shapes;
mod system;

pub struct ScoredEvent {
    pub side: component::Side,
}
pub struct PaddleCollisionEvent {
    pub side: component::Side,
    pub ball_entity: Entity,
}
pub struct StartPunchEvent {
    pub side: component::Side,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InGame,
    Pause,
    Won,
    Lost,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Such an unfair Multiball Pong!  My paddle is too small!!!1".to_string(),
            width: 1000.0,
            height: 600.0,
            ..Default::default()
        })
        .add_state(GameState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin);
    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<component::Velocity>()
        .register_inspectable::<component::Punch>()
        .register_inspectable::<component::PunchAnimation>()
        .register_inspectable::<component::Collider>();

    app.add_event::<ScoredEvent>()
        .add_event::<PaddleCollisionEvent>()
        .add_event::<StartPunchEvent>()
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(system::setup)
                .with_system(system::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(system::update_ui)
                .with_system(system::user_input)
                .with_system(system::movement)
                .with_system(system::ball_bounds_check)
                .with_system(system::paddle_collision)
                .with_system(system::score)
                .with_system(system::keep_paddle_in_screen)
                .with_system(system::ai)
                .with_system(system::punch::punch_animation)
                .with_system(system::punch::punch_increase)
                .with_system(system::punch::start_punch.label("start_punch"))
                .with_system(system::punch::punch_action.before("start_punch"))
                .with_system(system::ball_bounce_particles)
                .with_system(system::time_to_live)
                .with_system(system::punch_particles),
        )
        .add_system_set(SystemSet::on_update(GameState::Pause).with_system(system::pause))
        .add_system_set(SystemSet::on_enter(GameState::Won).with_system(system::setup_won))
        .add_system_set(SystemSet::on_update(GameState::Won).with_system(system::restart_on_space))
        .add_system_set(SystemSet::on_enter(GameState::Lost).with_system(system::setup_lost))
        .add_system_set(
            SystemSet::on_update(GameState::Lost).with_system(system::restart_on_space),
        );
    app.run();
}
