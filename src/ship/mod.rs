use crate::prelude::*;

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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0 , 16.0), 4, 1);
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

    if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
        sprite_index = 1;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
        sprite_index = 3;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
        sprite_index = 0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
        sprite_index = 2;
    }
    let new_position_y = ship_transform.translation.y + y_direction * 5.0 * time.delta_seconds();
    let new_position_x = ship_transform.translation.x + x_direction * 5.0 * time.delta_seconds();

    ship_transform.translation.y = dbg!(new_position_y);
    ship_transform.translation.x = dbg!(new_position_x);
    sprite.index = dbg!(sprite_index);
}