use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum Side {
    Left,
    Right,
}
#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Damping;

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
    pub fn left(&self, transform: &GlobalTransform) -> f32 {
        transform.translation.x - self.dimension.x
    }
    pub fn right(&self, transform: &GlobalTransform) -> f32 {
        transform.translation.x + self.dimension.x
    }
    pub fn top(&self, transform: &GlobalTransform) -> f32 {
        transform.translation.y + self.dimension.y
    }
    pub fn bottom(&self, transform: &GlobalTransform) -> f32 {
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