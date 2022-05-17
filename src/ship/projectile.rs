use crate::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(move_projectile)
            .add_system(destroy_projectile);
    }
}

#[derive(Component)]
struct Projectile {
    velocity: f32,
    direction: Vec2,
}

fn setup(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("projectile.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 1);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);
}

fn move_projectile() {

}

fn destroy_projectile() {

}