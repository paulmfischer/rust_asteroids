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
    asset_information: AssetInformation,
    controls: ShipControls,
    scale: f32,
}

impl ShipInformation {
    fn load() -> Self {
        let file = File::open("resources/ship.ron").expect("Failed to open ship file");
        from_reader(file).expect("Unable to load ship data")
    }
}

#[derive(Component, Deref, DerefMut)]
struct RotationTimer(Timer);

#[derive(Component)]
struct Ship {
    controls: ShipControls,
    max_speed: f32,
    velocity: f32,
    direction: Vec3,
}

impl Ship {
    fn new(controls: &ShipControls, max_speed: f32) -> Self {
        Ship {
            controls: controls.clone(),
            max_speed: max_speed,
            velocity: 0.0,
            direction: Vec3::new(0.0, 1.0, 1.0)
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

    let ship_component = Ship::new(&ship_info.controls, ship_info.max_speed);
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: ship_component.direction,
            scale: Vec3::splat(ship_info.scale),
            ..default()
        },
        // Transform::from_scale(Vec3::splat(1.5)),
        ..default()
    })
    .insert(ship_component)
    .insert(RotationTimer(Timer::from_seconds(0.1, true)));
}

fn move_ship(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RotationTimer, &mut Transform, &mut TextureAtlasSprite, &mut Ship), With<Ship>>,
) {
    let (mut rotation_timer, mut ship_transform, mut sprite, mut ship) = query.single_mut();
    let mut move_direction = ship_transform.translation;
    let mut sprite_index = sprite.index;

    rotation_timer.tick(time.delta());
    if rotation_timer.just_finished() {
        if keyboard_input.pressed(ship.controls.rotate_right) {
            if sprite_index == 7 {
                sprite_index = 0;
            } else {
                sprite_index += 1;
            }
            let mut new_direction_transform = dbg!(Transform::from_translation(ship.direction));
            new_direction_transform.rotate(dbg!(Quat::from_rotation_y(-45.0)));
            ship.direction = dbg!(new_direction_transform.translation);
            // .rotate(Quat::from_rotation_z(-45.0));
            
            // ship_transform.rotate(Quat::from_rotation_z(-45.0));
        }

        if keyboard_input.pressed(ship.controls.rotate_left) {
            if sprite_index == 0 {
                sprite_index = 7;
            } else {
                sprite_index -= 1;
            }
            // ship_transform.rotate(Quat::from_rotation_z(45.0));
        }

        sprite.index = sprite_index;
        // println!("ship translation {}, ship direction, {}", ship_transform.translation, ship.direction);
    }


    // if keyboard_input.pressed(ship.controls.forward) {
    //     // ship_transform.back();
    //     // move_direction = dbg!(ship_transform.forward() * ship.max_speed * time.delta_seconds());
    //     move_direction = dbg!(ship_transform.rotation * Vec2:: * ship.max_speed * time.delta_seconds());
    // }

    // ship_transform.translation = move_direction;

    // up & right
    // if keyboard_input.pressed(ship.controls.up) && keyboard_input.pressed(ship.controls.right) {
    //     x_direction += 0.75;
    //     y_direction += 0.75;
    //     sprite_index = 1;
    // }
    // // down & right
    // else if keyboard_input.pressed(ship.controls.down) && keyboard_input.pressed(ship.controls.right) {
    //     x_direction += 0.75;
    //     y_direction -= 0.75;
    //     sprite_index = 3;
    // }
    // // down & left
    // else if keyboard_input.pressed(ship.controls.down) && keyboard_input.pressed(ship.controls.left) {
    //     x_direction -= 0.75;
    //     y_direction -= 0.75;
    //     sprite_index = 5;
    // }
    // // up & left
    // else if keyboard_input.pressed(ship.controls.up) && keyboard_input.pressed(ship.controls.left) {
    //     x_direction -= 0.75;
    //     y_direction += 0.75;
    //     sprite_index = 7;
    // }
    // // up
    // else if keyboard_input.pressed(ship.controls.up) {
    //     y_direction += 1.0;
    //     sprite_index = 0;
    // }
    // // right
    // else if keyboard_input.pressed(ship.controls.right) {
    //     x_direction += 1.0;
    //     sprite_index = 2;
    // }
    // // down
    // else if keyboard_input.pressed(ship.controls.down) {
    //     y_direction -= 1.0;
    //     sprite_index = 4;
    // }
    // // left
    // else if keyboard_input.pressed(ship.controls.left) {
    //     x_direction -= 1.0;
    //     sprite_index = 6;
    // }

    // let new_position_y = ship_transform.translation.y + y_direction * ship.max_speed * time.delta_seconds();
    // let new_position_x = ship_transform.translation.x + x_direction * ship.max_speed * time.delta_seconds();

    // ship_transform.translation.y = new_position_y;
    // ship_transform.translation.x = new_position_x;
}


// fn animate_ship(
//     time: Res<Time>,
//     texture_atlas_assets: Res<Assets<TextureAtlas>>,
//     mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Ship>>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             let texture_atlas = texture_atlas_assets.get(texture_atlas_handle).unwrap();
//             sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
//         }
//     }
// }