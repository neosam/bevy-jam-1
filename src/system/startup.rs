use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::bundle;
use crate::component;
use crate::resource;
use crate::shapes as gameshapes;


pub fn setup(mut commands: Commands) {
    bevy::log::info!("Initialize");
    let game_globals = resource::GameGlobals::new();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(bundle::LeftPaddleBundle::new())
            .with_children(|parent| {
            parent.spawn_bundle(bundle::LeftSecondaryColliderBundle::new());
        }).insert(Name::new("Left Paddle"));
    commands.spawn_bundle(bundle::RightPaddleBundle::new())
        .insert(Name::new("Right Paddle"));
    commands.spawn_bundle(bundle::BallBundle::new(game_globals.ball_init_velocity))
        .insert(Name::new("Ball"));

    let digit_shapes = gameshapes::generate_digit_shapes(50.0);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(-100.0, 200.0, 1.0),
        ))
        .insert(component::LeftScoreUI)
        .insert(Name::new("Left Score"));
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(100.0, 200.0, 1.0),
        ))
        .insert(component::RightScoreUI)
        .insert(Name::new("Right Score"));

    commands.insert_resource(resource::Score::default());
    commands.insert_resource(digit_shapes);
    commands.insert_resource(resource::GameGlobals::new());
}


pub fn setup_won(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("You have won!");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_lost(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("You have lost!");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
