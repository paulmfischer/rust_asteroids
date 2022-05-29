use std::time::Duration;

use crate::prelude::*;

use super::Ship;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(create_projectile)
            .add_system(move_projectile)
            .add_system(destroy_projectile);
    }
}

#[derive(Deserialize)]
struct ProjectileInfo {
    velocity: f32,
    duration: f32,
    asset_info: AssetInformation,
}

impl ProjectileInfo {
    fn load() -> Self {
        let file = File::open("resources/projectile.ron").expect("Failed to open projectile file");
        from_reader(file).expect("Unable to deserialize projectile data")
    }
}


#[derive(Clone)]
struct ProjectileLoad {
    handle: Handle<TextureAtlas>,
    asset_info: AssetInformation,
    velocity: f32,
    duration: f32,
}

#[derive(Component)]
struct ProjectileTime {
    /// track when the projectile should evaporate (non-repeating timer)
    timer: Timer,
}

#[derive(Component)]
struct Projectile {
    velocity: f32,
    direction: Vec3,
}

impl Projectile {
    fn from(ship: &Ship, velocity: f32) -> Self {
        Projectile {
            velocity,
            direction: ship.rotation_direction,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let projectile_info = ProjectileInfo::load();
    let texture_handle = asset_server.load(&projectile_info.asset_info.sprite_image);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::from(projectile_info.asset_info.tile_size), projectile_info.asset_info.columns, projectile_info.asset_info.rows);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

    commands.insert_resource(ProjectileLoad {
        handle: texture_atlas_handle,
        asset_info: projectile_info.asset_info,
        velocity: projectile_info.velocity,
        duration: projectile_info.duration,
    });
}

fn create_projectile(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    projectile_load: Res<ProjectileLoad>,
    query: Query<(&Ship, &Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (ship, ship_transform) = query.single();
        let projectile_velocity = if ship.current_speed > 0.0 {
            projectile_load.velocity + ship.current_speed
        } else {
            projectile_load.velocity
        };
        let projectile = Projectile::from(ship, projectile_velocity);
        let projectile_position = 15.0;
        let starting_location = ship_transform.translation + Vec3::new(projectile.direction.x, projectile.direction.y, 1.0) * Vec3::new(projectile_position, projectile_position, 1.0);

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: projectile_load.handle.clone(),
            transform: Transform {
                translation: starting_location,
                rotation: Quat::from_rotation_arc(Vec3::Y, projectile.direction),
                scale: Vec3::new(1.0, projectile_load.asset_info.scale, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(projectile)
        .insert(ProjectileTime {
            timer: Timer::new(Duration::from_secs_f32(projectile_load.duration), false)
        });
    }
}

fn move_projectile(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(&mut Transform, &Projectile), With<Projectile>>
) {
    for (mut projectile_transform, projectile) in query.iter_mut() {
        let mut new_position_x = projectile_transform.translation.x + projectile.direction.x * projectile.velocity * time.delta_seconds();
        let mut new_position_y = projectile_transform.translation.y + projectile.direction.y * projectile.velocity * time.delta_seconds();

        // warp ship around to oppisite side if it crosses a boundary (window edge)
        if new_position_x > game_data.window.width_boundary {
            new_position_x = -game_data.window.width_boundary;
        } else if new_position_x < -game_data.window.width_boundary {
            new_position_x = game_data.window.width_boundary;
        }

        if new_position_y > game_data.window.height_boundary {
            new_position_y = -game_data.window.height_boundary;
        } else if new_position_y < -game_data.window.height_boundary {
            new_position_y = game_data.window.height_boundary;
        }

        projectile_transform.translation.x = new_position_x;
        projectile_transform.translation.y = new_position_y;
    }
}

fn destroy_projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ProjectileTime), With<Projectile>>
) {
    for (entity, mut projectile_timer) in query.iter_mut() {
        projectile_timer.timer.tick(time.delta());

        if projectile_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}