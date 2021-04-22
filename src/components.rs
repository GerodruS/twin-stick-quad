use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vector: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerController {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Visual {
    pub color: Color,
    pub shape: Shape,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Triangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DespawnWhenOffScreen {
    pub outer_bounds_radius: f32,
    pub despawn_delay: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bullet { }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Asteroid { }
