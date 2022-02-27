use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

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

#[derive(Component)]
pub enum Side {
    Left,
    Right,
}
#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AI;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            velocity: Vec2::new(x, y),
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub dimension: Vec2,
}
impl Collider {
    pub fn new(x: f32, y: f32) -> Self {
        Collider {
            dimension: Vec2::new(x, y),
        }
    }
    pub fn left(&self, transform: &Transform) -> f32 {
        transform.translation.x - self.dimension.x
    }
    pub fn right(&self, transform: &Transform) -> f32 {
        transform.translation.x + self.dimension.x
    }
    pub fn top(&self, transform: &Transform) -> f32 {
        transform.translation.y + self.dimension.y
    }
    pub fn bottom(&self, transform: &Transform) -> f32 {
        transform.translation.y - self.dimension.y
    }
    pub fn add(&self, other: &Collider) -> Collider {
        Collider {
            dimension: Vec2::new(
                self.dimension.x + other.dimension.x,
                self.dimension.y + other.dimension.y,
            ),
        }
    }
}

#[derive(Component)]
pub struct LeftScoreUI;

#[derive(Component)]
pub struct RightScoreUI;

pub struct ScoredEvent {
    pub side: Side,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InGame,
    Won,
    Lost,
}

#[derive(Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Default)]
pub struct DigitShapes {
    pub shapes: [Polygon; 10],
}

pub fn generate_digit_shapes(size: f32) -> DigitShapes {
    DigitShapes {
        shapes: [
            Polygon {
                points: vec![
                    Vec2::new(size, size),
                    Vec2::new(-size, size),
                    Vec2::new(-size, -size),
                    Vec2::new(size, -size),
                    Vec2::new(size, size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![Vec2::new(0.0, size), Vec2::new(0.0, -size)],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, size),
                    Vec2::new(size, size),
                    Vec2::new(size, 0.0),
                    Vec2::new(-size, 0.0),
                    Vec2::new(-size, -size),
                    Vec2::new(size, -size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, size),
                    Vec2::new(size, size),
                    Vec2::new(size, 0.0),
                    Vec2::new(-size, 0.0),
                    Vec2::new(size, 0.0),
                    Vec2::new(size, -size),
                    Vec2::new(-size, -size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, size),
                    Vec2::new(-size, 0.0),
                    Vec2::new(size, 0.0),
                    Vec2::new(size, size),
                    Vec2::new(size, -size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(size, size),
                    Vec2::new(-size, size),
                    Vec2::new(-size, 0.0),
                    Vec2::new(size, 0.0),
                    Vec2::new(size, -size),
                    Vec2::new(-size, -size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(size, size),
                    Vec2::new(-size, size),
                    Vec2::new(-size, -size),
                    Vec2::new(size, -size),
                    Vec2::new(size, 0.0),
                    Vec2::new(-size, 0.0),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, size),
                    Vec2::new(size, size),
                    Vec2::new(size, -size),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, size),
                    Vec2::new(size, size),
                    Vec2::new(size, -size),
                    Vec2::new(-size, -size),
                    Vec2::new(-size, size),
                    Vec2::new(-size, 0.0),
                    Vec2::new(size, 0.0),
                ],
                closed: false,
            },
            Polygon {
                points: vec![
                    Vec2::new(-size, -size),
                    Vec2::new(size, -size),
                    Vec2::new(size, size),
                    Vec2::new(-size, size),
                    Vec2::new(-size, 0.0),
                    Vec2::new(size, 0.0),
                ],
                closed: false,
            },
        ],
    }
}

pub fn setup_system(mut commands: Commands) {
    bevy::log::info!("Initialize");
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
        .insert(Player)
        .insert(Paddle)
        .insert(Side::Left)
        .insert(Velocity::new(0.0, 0.0))
        .insert(Collider::new(5.0, 15.0));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &pedal_right_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_xyz(450.0, 0.0, 100.0),
        ))
        .insert(Side::Right)
        .insert(Paddle)
        .insert(AI)
        .insert(Velocity::new(0.0, 0.0))
        .insert(Collider::new(5.0, 50.0));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &ball_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 100.0),
        ))
        .insert(Velocity {
            velocity: Vec2::new(-INIT_VELOCITY_X, INIT_VELOCITY_Y),
        })
        .insert(Collider::new(5.0, 5.0))
        .insert(Ball);

    let digit_shapes = generate_digit_shapes(50.0);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(-100.0, 200.0, 1.0),
        ))
        .insert(LeftScoreUI);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &digit_shapes.shapes[0],
            DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
            Transform::from_xyz(100.0, 200.0, 1.0),
        ))
        .insert(RightScoreUI);

    commands.insert_resource(Score::default());
    commands.insert_resource(digit_shapes);
}

pub fn setup_system_won(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("You have won!");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_system_lost(mut commands: Commands, query: Query<Entity>) {
    bevy::log::info!("You have lost!");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn user_input_system(
    mut query: Query<&mut Velocity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard.pressed(KeyCode::W) {
            velocity.velocity.y = PEDAL_MAX_SPEED
                .min(velocity.velocity.y + PEDAL_ACCELERATION * time.delta_seconds());
        }
        if keyboard.pressed(KeyCode::S) {
            velocity.velocity.y = (-PEDAL_MAX_SPEED)
                .max(velocity.velocity.y - PEDAL_ACCELERATION * time.delta_seconds());
        }
        if velocity.velocity.y > 0.0 {
            velocity.velocity.y =
                0.0f32.max(velocity.velocity.y - PEDAL_BREAK * time.delta_seconds());
        } else if velocity.velocity.y < 0.0 {
            velocity.velocity.y =
                0.0f32.min(velocity.velocity.y + PEDAL_BREAK * time.delta_seconds());
        }
    }
}

pub fn ai_system(
    mut ai_query: Query<(&mut Velocity, &Transform), With<AI>>,
    ball_query: Query<&Transform, With<Ball>>,
    time: Res<Time>,
) {
    for (mut velocity, transform) in ai_query.iter_mut() {
        let most_relevant_ball = ball_query
            .iter()
            .map(|ball_transform| (ball_transform.translation.x, ball_transform.translation.y))
            .fold(None, |acc, (x, y)| {
                if let Some((acc_x, acc_y)) = acc {
                    if x > acc_x {
                        Some((x, y))
                    } else {
                        Some((acc_x, acc_y))
                    }
                } else {
                    Some((x, y))
                }
            });
        if let Some((_, ball_y)) = most_relevant_ball {
            if ball_y < transform.translation.y {
                velocity.velocity.y = (-PEDAL_MAX_SPEED)
                    .max(velocity.velocity.y - PEDAL_ACCELERATION * time.delta_seconds());
            } else {
                velocity.velocity.y = PEDAL_MAX_SPEED
                    .min(velocity.velocity.y + PEDAL_ACCELERATION * time.delta_seconds());
            }
            if velocity.velocity.y > 0.0 {
                velocity.velocity.y =
                    0.0f32.max(velocity.velocity.y - PEDAL_BREAK * time.delta_seconds());
            } else if velocity.velocity.y < 0.0 {
                velocity.velocity.y =
                    0.0f32.min(velocity.velocity.y + PEDAL_BREAK * time.delta_seconds());
            }
        }
    }
}

pub fn movement_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}

pub fn ball_bounds_check_system(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    mut score_event_writer: EventWriter<ScoredEvent>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        if transform.translation.y < Y_BOUNDS_BOTTOM {
            transform.translation.y = Y_BOUNDS_BOTTOM;
            if velocity.velocity.y < 0.0 {
                velocity.velocity.y *= -1.0;
            }
        }
        if transform.translation.y > Y_BOUNDS_TOP {
            transform.translation.y = Y_BOUNDS_TOP;
            if velocity.velocity.y > 0.0 {
                velocity.velocity.y *= -1.0;
            }
        }
        if transform.translation.x < X_BOUNDS_LEFT {
            transform.translation.x = 0.0;
            velocity.velocity.x = INIT_VELOCITY_X;
            velocity.velocity.y = INIT_VELOCITY_Y;
            score_event_writer.send(ScoredEvent { side: Side::Right });
        }
        if transform.translation.x > X_BOUNDS_RIGHT {
            transform.translation.x = 0.0;
            velocity.velocity.x = INIT_VELOCITY_X;
            velocity.velocity.y = INIT_VELOCITY_Y;
            score_event_writer.send(ScoredEvent { side: Side::Left });
        }
    }
}

pub fn paddle_collision_system(
    mut ball_query: Query<(&mut Velocity, &Transform, &Collider), With<Ball>>,
    pedal_query: Query<(&Transform, &Side, &Collider), With<Paddle>>,
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
                    reflection_angle.cos() * ball_velocity.velocity.length() * BOUNCE_ACCELERATION;
                let new_velocity_y =
                    -reflection_angle.sin() * ball_velocity.velocity.length() * BOUNCE_ACCELERATION;

                match *pedal_side {
                    Side::Left => {
                        if ball_velocity.velocity.x < 0.0 {
                            ball_velocity.velocity.x = new_velocity_x;
                            ball_velocity.velocity.y = new_velocity_y;
                        }
                    }
                    Side::Right => {
                        if ball_velocity.velocity.x > 0.0 {
                            ball_velocity.velocity.x = -new_velocity_x;
                            ball_velocity.velocity.y = new_velocity_y;
                        }
                    }
                }
            }
        }
    }
}

pub fn keep_paddle_in_screen_system(
    mut query: Query<(&mut Transform, &mut Velocity, &Collider), With<Paddle>>,
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

pub fn score_system(
    mut commands: Commands,
    mut left_query: Query<&mut Path, (With<LeftScoreUI>, Without<RightScoreUI>)>,
    mut right_query: Query<&mut Path, (With<RightScoreUI>, Without<LeftScoreUI>)>,
    balls_query: Query<&Ball>,
    mut score_event_reader: EventReader<ScoredEvent>,
    mut score: ResMut<Score>,
    mut state: ResMut<State<GameState>>,
    digit_shapes: Res<DigitShapes>,
) {
    let mut changed = false;
    for event in score_event_reader.iter() {
        changed = true;
        match event.side {
            Side::Left => {
                score.left += 1;
                if score.left >= 10 {
                    state
                        .replace(GameState::Won)
                        .expect("Could not switch to winning state");
                }
            }
            Side::Right => {
                score.right += 1;
                if score.right >= 10 {
                    state
                        .replace(GameState::Lost)
                        .expect("Could not switch to loosing state");
                }
            }
        };
        let ball_shape = shapes::Rectangle {
            extents: Vec2::new(5.0, 5.0),
            origin: RectangleOrigin::Center,
        };
        if balls_query.iter().len() < MAX_BALLS {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &ball_shape,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::from_xyz(0.0, 0.0, 100.0),
                ))
                .insert(Velocity {
                    velocity: Vec2::new(-INIT_VELOCITY_X, INIT_VELOCITY_Y),
                })
                .insert(Collider::new(5.0, 5.0))
                .insert(Ball);
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
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_system))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(user_input_system)
                .with_system(movement_system)
                .with_system(ball_bounds_check_system)
                .with_system(paddle_collision_system)
                .with_system(score_system)
                .with_system(keep_paddle_in_screen_system)
                .with_system(ai_system),
        )
        .add_system_set(SystemSet::on_enter(GameState::Won).with_system(setup_system_won))
        .add_system_set(SystemSet::on_enter(GameState::Lost).with_system(setup_system_lost))
        .run();
}
