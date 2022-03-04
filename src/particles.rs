use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{component, bundle};


pub struct Random {
    a: u64,
    b: u64,
    m: u64,
    x: u64,
}

impl Random {
    pub fn new(init: u64) -> Self {
        Random {
            a: 214013,
            b: 2531011,
            m: 2147483648,
            x: init,
        }
    }

    pub fn gen_u64(&mut self) -> u64 {
        self.x = (self.a * self.x + self.b) % self.m;
        self.x
    }
    pub fn gen_f32(&mut self) -> f32 {
        self.gen_u64() as f32 / self.m as f32
    }
    pub fn gen_f32_range(&mut self, min: f32, max: f32) -> f32 {
        self.gen_f32() * (max - min) + min
    }
}

pub struct Particles {
    pub amount: u32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub color: Color,
    pub min_time_to_live: f32,
    pub max_time_to_live: f32,
}

impl Particles {
    pub fn generate(&self, commands: &mut Commands, mut random: Random, mut transform: Transform) {
        let shape = shapes::Rectangle {
            extents: Vec2::new(2.0, 2.0),
            origin: RectangleOrigin::Center,
        };
        transform.translation.z = 0.0;
        let Particles { amount, min_speed, max_speed, color , min_time_to_live, max_time_to_live} = *self;
        commands.spawn_batch((0..amount).map(move |_| {
            let speed = random.gen_f32_range(min_speed, max_speed);
            let speed_vec = Vec2::new(random.gen_f32() - 0.5, random.gen_f32() - 0.5);
            let ttl = random.gen_f32_range(min_time_to_live, max_time_to_live);
            bundle::ParticleBundle {
                shape_bundle: GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(color)),
                    transform.clone(),
                ),
                velocity: (speed_vec.normalize() * speed).into(),
                ttl: component::TimeToLive { ttl }
            }
        }));
    }
}