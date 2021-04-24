use legion::*;
use macroquad::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

#[macroquad::main("Asteroids")]
async fn main() {
    let game_settings = resources::GameSettings {
        resolution: vec2(800.0, 600.0),
        back_color: BLACK,
        player_color: WHITE,
        player_size: 20.0,
        player_max_speed: 200.0,
        player_texture: "assets/playerShip1_green.png".to_string(),
        bullet_color: RED,
        bullet_size: 10.0,
        asteroid_color: BLUE,
        asteroid_size: (10.0, 50.0),
        asteroid_spawn_delay: (0.0, 1.0),
        asteroid_speed: (10.0, 50.0),
        min_lifetime: 1.0,
    };

    let player_texture: Texture2D = load_texture(game_settings.player_texture.as_str()).await;
    let textures = vec![player_texture];

    let mut world = World::default();
    world.extend(vec![(
        // player entity
        components::Transform {
            position: Vec2::zero(),
            rotation: 0.0,
        },
        components::Velocity {
            vector: Vec2::zero(),
        },
        components::Visual {
            color: game_settings.player_color,
            shape: components::Shape::Sprite {
                texture: utils::TextureWrapper::new(0),
                size: Vec2::one() * game_settings.player_size,
            },
        },
        components::PlayerController {},
        components::Collider {
            radius: game_settings.player_size / 2.0,
        },
    )]);

    let mut schedule = Schedule::builder()
        // game logic
        .add_system(systems::control_rotation_system())
        .add_system(systems::move_transform_system())
        .add_system(systems::spawn_bullets_system())
        .add_system(systems::spawn_asteroids_system(1.0))
        .add_system(systems::bullets_vs_asteroids_system())
        .add_system(systems::despawn_objects_system())
        .add_system(systems::increase_lifetime_system())
        // render
        .add_system(systems::update_screen_size_system(0.0, Vec2::zero()))
        .add_system(systems::clear_screen_system())
        .add_system(systems::draw_visual_system())
        .add_system(systems::draw_fps_system(
            0,
            [0.0; systems::FPS_HISTORY_SIZE],
        ))
        .build();

    let mut resources = Resources::default();
    resources.insert(game_settings);
    resources.insert(resources::GlobalState {
        camera: Camera2D::default(),
        textures,
    });

    loop {
        let delta_time_seconds = get_frame_time();
        resources.insert(resources::DeltaTime {
            seconds: delta_time_seconds,
        });
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
