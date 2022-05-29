use crate::prelude::*;
use self::projectile::ProjectilePlugin;

mod projectile;
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ProjectilePlugin)
            .add_startup_system(setup)
            .add_system(move_ship);
    }
}

#[derive(Clone, Deserialize, Debug)]
struct ShipControls {
    forward: KeyCode,
    backwards: KeyCode,
    rotate_right: KeyCode,
    rotate_left: KeyCode,
    // shoot: KeyCode,
}

#[derive(Clone, Deserialize, Debug)]
struct ShipInformation {
    max_speed: f32,
    acceleration: f32,
    asset_information: AssetInformation,
    controls: ShipControls
}

impl ShipInformation {
    fn load() -> Self {
        let file = File::open("resources/ship.ron").expect("Failed to open ship file");
        from_reader(file).expect("Unable to load ship data")
    }
}

#[derive(Component, Deref, DerefMut)]
struct VelocityTimer(Timer);

#[derive(Component)]
struct Ship {
    controls: ShipControls,
    max_speed: f32,
    acceleration: f32,
    heading_direction: Vec3,
    current_speed: f32,
    rotation_speed: f32,
    rotation_direction: Vec3,
}

impl From<ShipInformation> for Ship {
    fn from(ship_info: ShipInformation) -> Self {
         Ship {
            controls: ship_info.controls,
            max_speed: ship_info.max_speed,
            acceleration: ship_info.acceleration,
            heading_direction: Vec3::Y,
            current_speed: 0.0,
            rotation_speed: f32::to_radians(360.0),
            rotation_direction: Vec3::Y,
        }
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let ship_info = ShipInformation::load();
    let texture_handle = asset_server.load(&ship_info.asset_information.sprite_image);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::from(ship_info.asset_information.tile_size), ship_info.asset_information.columns, ship_info.asset_information.rows);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 15.0), // put ship closer to be on top of projectiles
            scale: Vec3::splat(ship_info.asset_information.scale),
            ..default()
        },
        ..default()
    })
    .insert(Ship::from(ship_info))
    .insert(VelocityTimer(Timer::from_seconds(0.3, true)));
}

fn move_ship(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut VelocityTimer, &mut Ship, &mut Transform)>,
    game_data: Res<GameData>,
) {

    let (mut velocity_timer, mut ship, mut ship_transform) = query.single_mut();
    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(ship.controls.rotate_left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(ship.controls.rotate_right) {
        rotation_factor -= 1.0;
    }
    
    let rotation_delta = Quat::from_rotation_z(rotation_factor * ship.rotation_speed * time.delta_seconds());
    ship_transform.rotation *= rotation_delta;
    ship.rotation_direction = ship_transform.rotation * Vec3::Y;

    if keyboard_input.pressed(ship.controls.forward) {
        if ship.current_speed < 0.0 {
            ship.current_speed = 0.0;
        }
        ship.current_speed += ship.acceleration;
        ship.heading_direction = ship_transform.rotation * Vec3::Y;
    } else if keyboard_input.pressed(ship.controls.backwards) {
        if ship.current_speed > 0.0 {
            ship.current_speed = 0.0;
        }
        ship.current_speed -= ship.acceleration;
        ship.heading_direction = ship_transform.rotation * Vec3::Y;
    }
    
    ship.current_speed = ship.current_speed.clamp(-ship.max_speed, ship.max_speed);
    let movement_distance = ship.current_speed * time.delta_seconds();
    let translation_delta = ship.heading_direction * movement_distance;
    let mut new_translation = ship_transform.translation + translation_delta;
    
    velocity_timer.tick(time.delta());
    if velocity_timer.just_finished() {
        if ship.current_speed > 0.0 {
            ship.current_speed -= ship.acceleration;
        } else if ship.current_speed < 0.0 {
            ship.current_speed += ship.acceleration;
        }
    }

    // warp ship around to oppisite side if it crosses a boundary (window edge)
    if new_translation.x > game_data.window.width_boundary {
        new_translation.x = -game_data.window.width_boundary;
    } else if new_translation.x < -game_data.window.width_boundary {
        new_translation.x = game_data.window.width_boundary;
    }

    if new_translation.y > game_data.window.height_boundary {
        new_translation.y = -game_data.window.height_boundary;
    } else if new_translation.y < -game_data.window.height_boundary {
        new_translation.y = game_data.window.height_boundary;
    }
    
    ship_transform.translation = new_translation;
}