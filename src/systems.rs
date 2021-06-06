use legion::world::SubWorld;
use legion::*;
use macroquad::prelude::*;
use macroquad::audio::*;

use crate::components;
use crate::resources;
use crate::utils;

#[system]
pub fn update_screen_size(
    #[state] current_delay: &mut f32,
    #[state] last_screen_size: &mut Vec2,
    #[resource] delta_time: &resources::DeltaTime,
    #[resource] game_settings: &resources::GameSettings,
    #[resource] global_state: &mut resources::GlobalState,
) {
    const UPDATE_SCREEN_SIZE_DELAY_SECONDS: f32 = 1.0;

    let real_screen_size = vec2(screen_width(), screen_height());
    if real_screen_size == *last_screen_size {
        *current_delay = UPDATE_SCREEN_SIZE_DELAY_SECONDS;
    } else {
        *current_delay -= delta_time.seconds;
    }

    if *current_delay <= 0.0 {
        *last_screen_size = real_screen_size;

        let real_aspect_ratio = real_screen_size.x / real_screen_size.y;
        let target_aspect_ratio = game_settings.resolution.x / game_settings.resolution.y;
        let virtual_screen_size = if target_aspect_ratio < real_aspect_ratio {
            vec2(
                real_aspect_ratio * game_settings.resolution.y,
                game_settings.resolution.y,
            )
        } else {
            vec2(
                game_settings.resolution.x,
                game_settings.resolution.x / real_aspect_ratio,
            )
        };

        let camera = Camera2D {
            zoom: vec2(2.0 / virtual_screen_size.x, -2.0 / virtual_screen_size.y),
            target: Vec2::ZERO,
            ..Default::default()
        };
        set_camera(&camera);
        global_state.camera = camera;
    }
}

#[system]
pub fn clear_screen(#[resource] game_settings: &resources::GameSettings) {
    clear_background(WHITE);
    draw_rectangle(
        -game_settings.resolution.x / 2.0,
        -game_settings.resolution.y / 2.0,
        game_settings.resolution.x,
        game_settings.resolution.y,
        game_settings.back_color,
    );
}

pub const FPS_HISTORY_SIZE: usize = 60;

#[system]
pub fn draw_fps(
    #[state] history_index: &mut usize,
    #[state] history: &mut [f32; FPS_HISTORY_SIZE],
    #[resource] delta_time: &resources::DeltaTime,
) {
    *history_index = (*history_index + 1) % FPS_HISTORY_SIZE;
    history[*history_index] = delta_time.seconds;

    let mut max = f32::MIN;
    let mut avg = 0.0_f32;
    let mut min = f32::MAX;
    for delta_time in history.iter() {
        max = max.max(*delta_time);
        avg += *delta_time;
        min = min.min(*delta_time);
    }
    avg /= FPS_HISTORY_SIZE as f32;

    // TODO: add to settings
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("FPS")
            .show(egui_ctx, |ui| {
                ui.label(format!(
                    "{:.1} {:.1} {:.1}",
                    max * 1000.0,
                    avg * 1000.0,
                    min * 1000.0
                ));
            });
    });
}

#[system(for_each)]
#[filter(component::< components::PlayerController > ())]
pub fn control_rotation(
    transform: &mut components::Transform,
    velocity: &mut components::Velocity,
    #[resource] global_state: &resources::GlobalState,
    #[resource] game_settings: &resources::GameSettings,
) {
    let mouse_position = mouse_position();
    let mouse_position = global_state
        .camera
        .screen_to_world(vec2(mouse_position.0, mouse_position.1));
    let position = transform.position;
    let vector = (mouse_position - position).normalize();
    let angle = vector.angle_between(vec2(0.0, 1.0));
    transform.rotation = -angle.to_degrees();

    let mut velocity_value = Vec2::ZERO;
    velocity_value += move_by_key(KeyCode::W, KeyCode::Up, vec2(0.0, -1.0));
    velocity_value += move_by_key(KeyCode::A, KeyCode::Left, vec2(-1.0, 0.0));
    velocity_value += move_by_key(KeyCode::S, KeyCode::Down, vec2(0.0, 1.0));
    velocity_value += move_by_key(KeyCode::D, KeyCode::Right, vec2(1.0, 0.0));
    if velocity_value != Vec2::ZERO {
        velocity_value = velocity_value.normalize();
    }
    velocity.vector = velocity_value * game_settings.player_max_speed;

    fn move_by_key(code_a: KeyCode, code_b: KeyCode, vec: Vec2) -> Vec2 {
        if is_key_down(code_a) || is_key_down(code_b) {
            vec
        } else {
            Vec2::ZERO
        }
    }
}

#[system(for_each)]
#[filter(component::< components::PlayerController > ())]
pub fn spawn_bullets(
    #[state] delay: &mut f32,
    transform: &mut components::Transform,
    #[resource] delta_time: &resources::DeltaTime,
    #[resource] game_settings: &resources::GameSettings,
    #[resource] global_state: &resources::GlobalState,
    cmd: &mut legion::systems::CommandBuffer,
) {
    if 0.0 < *delay {
        *delay -= delta_time.seconds;
    } else if is_key_down(KeyCode::Space) {
        let rotation = Mat2::from_angle(transform.rotation.to_radians());
        let velocity = rotation * vec2(0.0, 1.0);

        let sound = macroquad::rand::gen_range(
            0,
            global_state.bullet_spawn_sounds.len(),
        );
        let sound = global_state.bullet_spawn_sounds[sound];
        play_sound_once(sound);

        cmd.push((
            components::Bullet {},
            components::Transform {
                position: transform.position,
                rotation: transform.rotation,
            },
            components::Velocity {
                vector: velocity * 100.0,
            },
            components::Visual {
                color: game_settings.bullet_color,
                shape: components::Shape::Sprite {
                    texture: utils::TextureWrapper::new(0, game_settings.bullet_texture_rect),
                    size: Vec2::ONE * game_settings.bullet_size,
                },
            },
            components::DespawnWhenOffScreen {
                outer_bounds_radius: game_settings.bullet_size,
                despawn_delay: 0.0,
            },
            components::Collider {
                radius: game_settings.bullet_size / 2.0,
            },
        ));
        *delay = game_settings.bullet_delay;
    }
}

#[system]
pub fn spawn_asteroids(
    #[state] timer: &mut f32,
    #[resource] game_settings: &resources::GameSettings,
    #[resource] delta_time: &resources::DeltaTime,
    cmd: &mut legion::systems::CommandBuffer,
) {
    if *timer < 0.0 {
        let radius = macroquad::rand::gen_range(
            game_settings.asteroid_size.0,
            game_settings.asteroid_size.1,
        );
        let outer_bounds_radius = radius;

        let mut angle =
            macroquad::rand::gen_range(-std::f32::consts::PI / 4.0, std::f32::consts::PI / 4.0);
        let position = macroquad::rand::gen_range(
            0.0,
            2.0 * (game_settings.resolution.x + game_settings.resolution.y),
        );
        let position = {
            let mut position = position;
            if position < game_settings.resolution.x {
                vec2(position, -outer_bounds_radius) // top
            } else {
                position -= game_settings.resolution.x;
                if position < game_settings.resolution.y {
                    angle += std::f32::consts::PI / 2.0;
                    vec2(game_settings.resolution.x + outer_bounds_radius, position)
                // right
                } else {
                    position -= game_settings.resolution.y;
                    if position < game_settings.resolution.x {
                        angle += std::f32::consts::PI;
                        vec2(position, game_settings.resolution.y + outer_bounds_radius)
                    // bottom
                    } else {
                        position -= game_settings.resolution.x;
                        angle -= std::f32::consts::PI / 2.0;
                        vec2(-outer_bounds_radius, position) // left
                    }
                }
            }
        };
        let position = position - game_settings.resolution / 2.0;

        let rotation = Mat2::from_angle(angle);
        let velocity = rotation * vec2(0.0, 1.0);
        let angular_velocity = macroquad::rand::gen_range(
            -game_settings.asteroid_max_angular_velocity,
            game_settings.asteroid_max_angular_velocity,
        );
        let speed = macroquad::rand::gen_range(
            game_settings.asteroid_speed.0,
            game_settings.asteroid_speed.1,
        );
        let rect_index = macroquad::rand::gen_range(
            0,
            game_settings.asteroids_texture_rect.len(),
        );
        let rect = game_settings.asteroids_texture_rect[rect_index];
        cmd.push((
            components::Asteroid {},
            components::Transform {
                position,
                rotation: 0.0,
            },
            components::Velocity {
                vector: velocity * speed,
            },
            components::AngularVelocity {
                value: angular_velocity,
            },
            components::Visual {
                color: game_settings.asteroid_color,
                shape: components::Shape::Sprite {
                    texture: utils::TextureWrapper::new(0, rect),
                    size: Vec2::ONE * 2.0 * radius,
                },
            },
            components::DespawnWhenOffScreen {
                outer_bounds_radius: radius,
                despawn_delay: 0.0,
            },
            components::Collider { radius },
        ));

        *timer = macroquad::rand::gen_range(
            game_settings.asteroid_spawn_delay.0,
            game_settings.asteroid_spawn_delay.1,
        );
    } else {
        *timer -= delta_time.seconds;
    }
}

#[system(for_each)]
pub fn despawn_objects(
    transform: &components::Transform,
    despawn: &components::DespawnWhenOffScreen,
    entity: &Entity,
    #[resource] game_settings: &resources::GameSettings,
    cmd: &mut legion::systems::CommandBuffer,
) {
    if game_settings.min_lifetime < despawn.despawn_delay {
        let position = transform.position;
        let position = position + game_settings.resolution / 2.0;
        let outer_bounds_radius = despawn.outer_bounds_radius;
        if position.x < -outer_bounds_radius
            || game_settings.resolution.x + outer_bounds_radius < position.x
            || position.y < -outer_bounds_radius
            || game_settings.resolution.y + outer_bounds_radius < position.y
        {
            cmd.remove(*entity);
        }
    }
}

#[system(for_each)]
pub fn move_transform(
    transform: &mut components::Transform,
    velocity: &components::Velocity,
    #[resource] delta_time: &resources::DeltaTime,
) {
    transform.position += velocity.vector * delta_time.seconds;
}

#[system(for_each)]
pub fn rotate_transform(
    transform: &mut components::Transform,
    angular_velocity: &components::AngularVelocity,
    #[resource] delta_time: &resources::DeltaTime,
) {
    transform.rotation += angular_velocity.value * delta_time.seconds;
}

#[system(for_each)]
pub fn increase_lifetime(
    despawn: &mut components::DespawnWhenOffScreen,
    #[resource] delta_time: &resources::DeltaTime,
) {
    despawn.despawn_delay += delta_time.seconds;
}

#[system(for_each)]
pub fn draw_visual(
    transform: &components::Transform,
    visual: &components::Visual,
    #[resource] global_state: &mut resources::GlobalState,
) {
    match visual.shape {
        components::Shape::Triangle { width, height } => {
            let a = vec2(0.0, height / 2.0);
            let b = vec2(width / 2.0, -height / 2.0);
            let c = vec2(-width / 2.0, -height / 2.0);

            let rotation = Mat2::from_angle(transform.rotation.to_radians());
            let a = rotation * a;
            let b = rotation * b;
            let c = rotation * c;

            let position = transform.position;
            let a = a + position;
            let b = b + position;
            let c = c + position;

            draw_triangle(a, b, c, visual.color);
        }
        components::Shape::Circle { radius } => {
            let position = transform.position;
            draw_circle(position.x, position.y, radius, visual.color)
        }
        components::Shape::Sprite { texture, size } => {
            let position = transform.position;
            let rotation = (transform.rotation + 180.0).to_radians();

            let rect = texture.rect;
            let texture = global_state.textures[texture.index];
            draw_texture_ex(
                texture,
                position.x - size.x / 2.0,
                position.y - size.y / 2.0,
                visual.color,
                DrawTextureParams {
                    dest_size: Some(size),
                    source: Some(rect),
                    rotation,
                    ..Default::default()
                },
            );
        }
    }
}

#[system(for_each)]
#[read_component(components::Bullet)]
#[filter(component::< components::Asteroid > ())]
pub fn bullets_vs_asteroids(
    transform: &components::Transform,
    collider: &components::Collider,
    entity: &Entity,
    world: &SubWorld,
    cmd: &mut legion::systems::CommandBuffer,
) {
    let position = transform.position;
    let radius = collider.radius;
    let mut remove = false;

    let mut query = <(&components::Transform, &components::Collider, Entity)>::query()
        .filter(component::<components::Bullet>());
    for (t, c, e) in query.iter(world) {
        let distance_sqr = (position - t.position).length_squared();
        let radius_sqr = radius + c.radius;
        let radius_sqr = radius_sqr * radius_sqr;
        if distance_sqr < radius_sqr {
            remove = true;
            cmd.remove(*e);
        }
    }

    if remove {
        cmd.remove(*entity);
    }
}
