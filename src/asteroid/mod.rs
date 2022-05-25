use crate::prelude::*;

const MAX_ASTEROIDS: i8 = 2;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

#[derive(Component)]
struct Asteroid {
    velocity: f32,
    direction: Vec2,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    // game_data: Res<GameData>,
) {
    let texture_handle = asset_server.load("asteroid.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 1);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

    for count in 1..=MAX_ASTEROIDS {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-100.0 * (-1.0 * f32::from(count)), 200.0, 10.0),
                scale: Vec3::splat(8.0),
                ..default()
            },
            ..default()
        })
        .insert(Asteroid {
            velocity: 45.0,
            direction: Vec2::new(-1.0, 0.0)
        });
    }
}