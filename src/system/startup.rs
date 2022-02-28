use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::component;
use crate::resource;
use crate::shapes as gameshapes;


pub fn setup(mut commands: Commands) {
    bevy::log::info!("Initialize");
    let game_globals = resource::GameGlobals::new();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let pedal_left_shape = shapes::Rectangle {
        extents: Vec2::new(5.0, 20.0),
        origin: RectangleOrigin::Center,
    };
    let pedal_right_shape = shapes::Rectangle {
        extents: Vec2::new(5.0, 100.0),
        origin: RectangleOrigin::Center,
    };
    let ball_shape = shapes::Rectangle {
        extents: Vec2::new(5.0, 5.0),
        origin: RectangleOrigin::Center,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &pedal_left_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_xyz(-450.0, 0.0, 100.0),
        ))
        .insert(component::Player)
        .insert(component::Paddle)
        .insert(component::Side::Left)
        .insert(component::Velocity::new(0.0, 0.0))
        .insert(component::Collider::new(5.0, 15.0));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &pedal_right_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_xyz(450.0, 0.0, 100.0),
        ))
        .insert(component::Side::Right)
        .insert(component::Paddle)
        .insert(component::AI)
        .insert(component::Velocity::new(0.0, 0.0))
        .insert(component::Collider::new(5.0, 50.0));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &ball_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 100.0),
        ))
        .insert(component::Velocity {
            velocity: game_globals.ball_init_velocity.clone(),
        })
        .insert(component::Collider::new(5.0, 5.0))
        .insert(component::Ball);

    let digit_shapes = gameshapes::generate_digit_shapes(50.0);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(-100.0, 200.0, 1.0),
        ))
        .insert(component::LeftScoreUI);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(100.0, 200.0, 1.0),
        ))
        .insert(component::RightScoreUI);

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
