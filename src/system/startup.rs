use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::bundle;
use crate::component;
use crate::resource;
use crate::shapes as gameshapes;


pub fn setup(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("Initialize");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
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
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let letters = crate::shapes::generate_letter_shapes(50.0);

    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.y,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-110.0, 110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.o,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-0.0, 110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.u,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(110.0, 110.0, 1.0),
    ));

    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.w,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-110.0, -110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.o,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-0.0, -110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.n,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(110.0, -110.0, 1.0),
    ));
}

pub fn setup_lost(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("You have lost!");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let letters = crate::shapes::generate_letter_shapes(50.0);

    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.y,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-110.0, 110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.o,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-0.0, 110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.u,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(110.0, 110.0, 1.0),
    ));

    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.l,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-170.0, -110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.o,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(-60.0, -110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.s,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(60.0, -110.0, 1.0),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &letters.t,
        DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
        Transform::from_xyz(170.0, -110.0, 1.0),
    ));
}
