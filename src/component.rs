use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Copy, Clone, PartialEq, Eq, Inspectable)]
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

#[derive(Component, Inspectable)]
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
impl From<Vec2> for Velocity {
    fn from(vec2: Vec2) -> Self {
        Velocity::new(vec2.x, vec2.y)
    }
}

#[derive(Component, Inspectable)]
pub struct Punch {
    pub strength: f32,
    pub max_strength: f32,
}
impl Punch {
    pub fn new(max_strength: f32) -> Self {
        Punch {
            strength: 0.0,
            max_strength,
        }
    }
}

#[derive(Component, Inspectable)]
pub struct PunchAnimation {
    pub duration: f32,
    pub distance: f32,
    pub base_x: f32,
}

#[derive(Component, Inspectable)]
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
pub struct ColliderOnly;

#[derive(Component)]
pub struct LeftScoreUI;

#[derive(Component)]
pub struct RightScoreUI;

#[derive(Component)]
pub struct StrengthUI;

#[derive(Component)]
pub struct TimeToLive {
    pub ttl: f32,
}