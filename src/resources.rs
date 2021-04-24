use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeltaTime {
    pub seconds: f32,
}

pub struct GameSettings {
    pub resolution: Vec2,
    pub back_color: Color,
    pub player_color: Color,
    pub player_size: f32,
    pub player_max_speed: f32,
    pub player_texture: String,
    pub bullet_color: Color,
    pub bullet_size: f32,
    pub asteroid_color: Color,
    pub asteroid_size: (f32, f32),
    pub asteroid_spawn_delay: (f32, f32),
    pub asteroid_speed: (f32, f32),
    pub min_lifetime: f32,
}

pub struct GlobalState {
    pub camera: Camera2D,
    pub textures: Vec<Texture2D>,
}
