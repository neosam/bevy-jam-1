use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, entity::ShapeBundle};
use crate::component;

#[derive(Bundle)]
pub struct PaddleBundle {
    #[bundle]
    shape_bundle: ShapeBundle,
    paddle: component::Paddle,
    side: component::Side,
    damping: component::Damping,
    velocity: component::Velocity,
    collider: component::Collider,
}
impl PaddleBundle {
    pub fn new(side: component::Side) -> Self {
        let (paddle_shape, x, collider) = match side {
            component::Side::Left => {
                (shapes::Rectangle {
                        extents: Vec2::new(5.0, 20.0),
                        origin: RectangleOrigin::Center,
                    },
                    -450.,
                    component::Collider::new(5.0, 15.0),
                )
            },
            component::Side::Right => {
                (shapes::Rectangle {
                        extents: Vec2::new(5.0, 100.0),
                        origin: RectangleOrigin::Center,
                    },
                    450.,
                    component::Collider::new(5.0, 50.0),
                )
            },
        };
        PaddleBundle {
            shape_bundle: GeometryBuilder::build_as(
                &paddle_shape,
                DrawMode::Fill(FillMode::color(Color::WHITE)),
                Transform::from_xyz(x, 0.0, 100.0),
            ),
            paddle: component::Paddle,
            side,
            damping: component::Damping,
            velocity: component::Velocity::new(0.0, 0.0),
            collider,
        }
    }
}

#[derive(Bundle)]
pub struct LeftPaddleBundle {
    #[bundle]
    paddle_bundle: PaddleBundle,
    player: component::Player,
    punch: component::Punch,
}
impl LeftPaddleBundle {
    pub fn new() -> Self {
        LeftPaddleBundle {
            paddle_bundle: PaddleBundle::new(component::Side::Left),
            player: component::Player,
            punch: component::Punch::new(3.0),
        }
    }
}

#[derive(Bundle)]
pub struct RightPaddleBundle {
    #[bundle]
    paddle_bundle: PaddleBundle,
    ai: component::AI,
}
impl RightPaddleBundle {
    pub fn new() -> Self {
        RightPaddleBundle {
            paddle_bundle: PaddleBundle::new(component::Side::Right),
            ai: component::AI,
        }
    }
}

#[derive(Bundle)]
pub struct LeftSecondaryColliderBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    side: component::Side,
    paddle: component::Paddle,
    collider: component::Collider,
    collider_only: component::ColliderOnly,
}
impl LeftSecondaryColliderBundle {
    pub fn new() -> Self {
        LeftSecondaryColliderBundle {
            transform: Transform::from_xyz(-20.0, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
            side: component::Side::Left,
            paddle: component::Paddle,
            collider: component::Collider::new(15.0, 15.0),
            collider_only: component::ColliderOnly,
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    #[bundle]
    shape_bundle: ShapeBundle,

    velocity: component::Velocity,
    collider: component::Collider,
    ball: component::Ball,
}
impl BallBundle {
    pub fn new(velocity: Vec2) -> Self {
        let ball_shape = shapes::Rectangle {
            extents: Vec2::new(5.0, 5.0),
            origin: RectangleOrigin::Center,
        };
        BallBundle { 
            shape_bundle: GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Fill(FillMode::color(Color::WHITE)),
                Transform::from_xyz(0.0, 0.0, 100.0),
            ),
            velocity: component::Velocity::new(velocity.x, velocity.y),
            collider: component::Collider::new(5.0, 5.0),
            ball: component::Ball,
        }
    }
}


