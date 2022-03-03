use bevy::prelude::*;
use bevy_prototype_lyon::shapes::Polygon;

#[derive(Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Default)]
pub struct DigitShapes {
    pub shapes: [Polygon; 10],
}

pub struct LetterShapes {
    pub y: Polygon,
    pub o: Polygon,
    pub u: Polygon,
    pub w: Polygon,
    pub n: Polygon,
    pub l: Polygon,
    pub s: Polygon,
    pub t: Polygon,
}

pub struct GameGlobals {
    pub bounds_left: f32,
    pub bounds_right: f32,
    pub bounds_top: f32,
    pub bounds_bottom: f32,
    pub paddle_acceleration: f32,
    pub paddle_break: f32,
    pub paddle_max_speed: f32,
    pub speed_increase_per_bounce: f32,
    pub ball_init_velocity: Vec2,
    pub max_balls: usize,
}
impl GameGlobals {
    pub fn new() -> Self {
        GameGlobals {
            bounds_left: -500.0,
            bounds_right: 500.0,
            bounds_top: 300.0,
            bounds_bottom: -275.0,
            paddle_acceleration: 10000.0,
            paddle_break: 9000.0,
            paddle_max_speed: 400.0,
            speed_increase_per_bounce: 1.2,
            ball_init_velocity: Vec2::new(120.0, 90.0),
            max_balls: 4,
        }
    }
}