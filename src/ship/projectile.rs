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
}

#[derive(Component)]
struct Projectile {
    velocity: f32,
    direction: Vec2,
}

impl Projectile {
    fn from(ship: &Ship, velocity: f32) -> Self {
        Projectile {
            velocity,
            direction: ship.direction,
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
        let projectile = Projectile::from(ship, projectile_load.velocity);
        let projectile_position = 15.0;
        let starting_location = ship_transform.translation + Vec3::new(projectile.direction.x, projectile.direction.y, 1.0) * Vec3::new(projectile_position, projectile_position, 1.0);

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: projectile_load.handle.clone(),
            transform: Transform {
                translation: starting_location,
                rotation: Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction),
                scale: Vec3::new(1.0, projectile_load.asset_info.scale, 1.0), // Vec3::splat(projectile_load.asset_info.scale),
                ..default()
            },
            ..default()
        })
        .insert(projectile);
    }
}

fn move_projectile(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Projectile), With<Projectile>>
) {
    for (mut projectile_transform, projectile) in query.iter_mut() {
        let new_position_x = projectile_transform.translation.x + projectile.direction.x * projectile.velocity * time.delta_seconds();
        let new_position_y = projectile_transform.translation.y + projectile.direction.y * projectile.velocity * time.delta_seconds();
    
        projectile_transform.translation.x = new_position_x;
        projectile_transform.translation.y = new_position_y;
    }
}

fn destroy_projectile(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(&Transform, Entity), With<Projectile>>
) {
    let window = windows.primary();
    let height_boundary = window.height() / 2.0;
    let width_boundary = window.width() / 2.0;

    for (proj_trans, entity) in query.iter() {
        if proj_trans.translation.x.abs() > width_boundary || proj_trans.translation.y.abs() > height_boundary {
            commands.entity(entity).despawn();
        }
    }
}