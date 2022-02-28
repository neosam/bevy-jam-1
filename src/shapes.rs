use bevy::prelude::*;
use bevy_prototype_lyon::shapes::Polygon;
use crate::resource;

pub fn generate_digit_shapes(size: f32) -> resource::DigitShapes {
    resource::DigitShapes {
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