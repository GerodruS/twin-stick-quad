use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeltaTime {
    pub seconds: f32,
}

pub struct GameSettings {
    pub resolution: Vec2,
    pub back_color: Color,
    pub sprites_sheet: String,
    pub player_color: Color,
    pub player_size: f32,
    pub player_max_speed: f32,
    pub player_texture_rect: Rect,
    pub bullet_color: Color,
    pub bullet_size: f32,
    pub bullet_texture_rect: Rect,
    pub asteroid_color: Color,
    pub asteroid_size: (f32, f32),
    pub asteroid_spawn_delay: (f32, f32),
    pub asteroid_speed: (f32, f32),
    pub asteroids_texture_rect: Vec::<Rect>,
    pub min_lifetime: f32,
}

pub struct GlobalState {
    pub camera: Camera2D,
    pub textures: Vec<Texture2D>,
}
