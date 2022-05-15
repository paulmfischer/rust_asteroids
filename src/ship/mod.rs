use crate::prelude::*;

const SHIP_SPEED: f32 = 25.0;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            // .add_system(animate_ship)
            .add_system(move_ship)
            ;
    }
}

#[derive(Component)]
struct Ship {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("ship.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0 , 16.0), 8, 1);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(1.5)),
        ..default()
    })
    .insert(Ship {})
    .insert(AnimationTimer(Timer::from_seconds(0.3, true)));
}

fn animate_ship(
    time: Res<Time>,
    texture_atlas_assets: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Ship>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlas_assets.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_ship(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Ship>>,
) {
    let mut y_direction = 0.0;
    let mut x_direction = 0.0;
    let (mut ship_transform, mut sprite) = query.single_mut();
    let mut sprite_index = sprite.index;

    // up & right
    if keyboard_input.pressed(KeyCode::W) && keyboard_input.pressed(KeyCode::D) {
        x_direction += 1.0;
        y_direction += 1.0;
        sprite_index = 1;
    }
    // down & right
    else if keyboard_input.pressed(KeyCode::S) && keyboard_input.pressed(KeyCode::D) {
        x_direction += 1.0;
        y_direction -= 1.0;
        sprite_index = 3;
    }
    // down & left
    else if keyboard_input.pressed(KeyCode::S) && keyboard_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
        y_direction -= 1.0;
        sprite_index = 5;
    }
    // up & left
    else if keyboard_input.pressed(KeyCode::W) && keyboard_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
        y_direction += 1.0;
        sprite_index = 7;
    }
    // up
    else if keyboard_input.pressed(KeyCode::W) {
        y_direction += 1.0;
        sprite_index = 0;
    }
    // right
    else if keyboard_input.pressed(KeyCode::D) {
        x_direction += 1.0;
        sprite_index = 2;
    }
    // down
    else if keyboard_input.pressed(KeyCode::S) {
        y_direction -= 1.0;
        sprite_index = 4;
    }
    // left
    else if keyboard_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
        sprite_index = 6;
    }

    let new_position_y = ship_transform.translation.y + y_direction * SHIP_SPEED * time.delta_seconds();
    let new_position_x = ship_transform.translation.x + x_direction * SHIP_SPEED * time.delta_seconds();

    ship_transform.translation.y = new_position_y;
    ship_transform.translation.x = new_position_x;
    sprite.index = sprite_index;
}