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