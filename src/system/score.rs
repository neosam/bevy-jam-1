pub use bevy::prelude::*;
pub use bevy_prototype_lyon::prelude::*;
use crate::{component, resource};

pub fn score(
    mut commands: Commands,
    mut left_query: Query<&mut Path, (With<component::LeftScoreUI>, Without<component::RightScoreUI>)>,
    mut right_query: Query<&mut Path, (With<component::RightScoreUI>, Without<component::LeftScoreUI>)>,
    balls_query: Query<&component::Ball>,
    mut score_event_reader: EventReader<crate::ScoredEvent>,
    mut score: ResMut<resource::Score>,
    mut state: ResMut<State<crate::GameState>>,
    digit_shapes: Res<resource::DigitShapes>,
) {
    let mut changed = false;
    for event in score_event_reader.iter() {
        changed = true;
        match event.side {
            component::Side::Left => {
                score.left += 1;
                if score.left >= 10 {
                    state
                        .replace(crate::GameState::Won)
                        .expect("Could not switch to winning state");
                }
            }
            component::Side::Right => {
                score.right += 1;
                if score.right >= 10 {
                    state
                        .replace(crate::GameState::Lost)
                        .expect("Could not switch to loosing state");
                }
            }
        };
        let ball_shape = shapes::Rectangle {
            extents: Vec2::new(5.0, 5.0),
            origin: RectangleOrigin::Center,
        };
        if balls_query.iter().len() < crate::MAX_BALLS {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &ball_shape,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::from_xyz(0.0, 0.0, 100.0),
                ))
                .insert(component::Velocity {
                    velocity: Vec2::new(-crate::INIT_VELOCITY_X, crate::INIT_VELOCITY_Y),
                })
                .insert(component::Collider::new(5.0, 5.0))
                .insert(component::Ball);
        }
    }
    if changed {
        if let Ok(mut left_digit_path) = left_query.get_single_mut() {
            *left_digit_path =
                ShapePath::build_as(&digit_shapes.shapes[(score.left % 10) as usize]);
        }
        if let Ok(mut right_digit_path) = right_query.get_single_mut() {
            *right_digit_path =
                ShapePath::build_as(&digit_shapes.shapes[(score.right % 10) as usize]);
        }
    }
}