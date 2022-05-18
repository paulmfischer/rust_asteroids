use crate::prelude::*;
use self::projectile::ProjectilePlugin;

mod projectile;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ProjectilePlugin)
            .add_startup_system(setup)
            .add_system(move_ship)
            ;
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
    velocity: f32,
    acceleration: f32,
    direction: Vec2,
}

impl From<ShipInformation> for Ship {
    fn from(ship_info: ShipInformation) -> Self {
         Ship {
            controls: ship_info.controls,
            max_speed: ship_info.max_speed,
            velocity: 0.0,
            direction: Vec2::Y,
            acceleration: ship_info.acceleration,
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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::from(ship_info.asset_information.tile_size), ship_info.asset_information.columns, ship_info.asset_information.rows);// Vec2::new(16.0 , 16.0), 8, 1);
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
    game_data: Res<GameData>,
    mut query: Query<(&mut VelocityTimer, &mut Transform, &mut TextureAtlasSprite, &mut Ship), With<Ship>>,
) {
    let (mut velocity_timer, mut ship_transform, mut sprite, mut ship) = query.single_mut();
    let mut sprite_index = sprite.index;
    let mut y_direction = 0.0;
    let mut x_direction = 0.0;

    // up & right
    if keyboard_input.pressed(ship.controls.forward) && keyboard_input.pressed(ship.controls.rotate_right) {
        x_direction += 0.75;
        y_direction += 0.75;
        ship.direction = Vec2::new(0.75, 0.75);
        sprite_index = 1;
    }
    // down & right
    else if keyboard_input.pressed(ship.controls.backwards) && keyboard_input.pressed(ship.controls.rotate_right) {
        x_direction += 0.75;
        y_direction -= 0.75;
        ship.direction = Vec2::new(0.75, -0.75);
        sprite_index = 3;
    }
    // down & left
    else if keyboard_input.pressed(ship.controls.backwards) && keyboard_input.pressed(ship.controls.rotate_left) {
        x_direction -= 0.75;
        y_direction -= 0.75;
        ship.direction = Vec2::new(-0.75, -0.75);
        sprite_index = 5;
    }
    // up & left
    else if keyboard_input.pressed(ship.controls.forward) && keyboard_input.pressed(ship.controls.rotate_left) {
        x_direction -= 0.75;
        y_direction += 0.75;
        ship.direction = Vec2::new(-0.75, 0.75);
        sprite_index = 7;
    }
    // up
    else if keyboard_input.pressed(ship.controls.forward) {
        y_direction += 1.0;
        ship.direction = Vec2::new(0.0, 1.0);
        sprite_index = 0;
    }
    // right
    else if keyboard_input.pressed(ship.controls.rotate_right) {
        x_direction += 1.0;
        ship.direction = Vec2::new(1.0, 0.0);
        sprite_index = 2;
    }
    // down
    else if keyboard_input.pressed(ship.controls.backwards) {
        y_direction -= 1.0;
        ship.direction = Vec2::new(0.0, -1.0);
        sprite_index = 4;
    }
    // left
    else if keyboard_input.pressed(ship.controls.rotate_left) {
        x_direction -= 1.0;
        ship.direction = Vec2::new(-1.0, 0.0);
        sprite_index = 6;
    }

    velocity_timer.tick(time.delta());
    if velocity_timer.just_finished() {
        if y_direction == 0.0 && x_direction == 0.0 {
            ship.velocity = (ship.velocity - ship.acceleration).clamp(0.0, ship.max_speed);
        } else {
            ship.velocity = (ship.velocity + ship.acceleration).clamp(0.0, ship.max_speed);
        }
    }

    let mut new_position_y = ship_transform.translation.y + ship.direction.y * ship.velocity * time.delta_seconds();
    let mut new_position_x = ship_transform.translation.x + ship.direction.x * ship.velocity * time.delta_seconds();

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

    ship_transform.translation.y = new_position_y;
    ship_transform.translation.x = new_position_x;
    sprite.index = sprite_index;
}