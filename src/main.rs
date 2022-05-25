use std::fs::File;
use bevy::{
    prelude::*,
    window::WindowMode,
};
use ron::de::from_reader;
use serde::Deserialize;
use ship::ShipPlugin;
use asteroid::AsteroidPlugin;

mod ship;
mod asteroid;

pub mod prelude {
    pub use std::fs::File;
    pub use bevy::prelude::*;
    pub use ron::de::from_reader;
    pub use serde::Deserialize;
    pub use crate::GameState;
    pub use crate::AssetInformation;
    pub use crate::GameData;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
    Exit,
}

pub struct WindowData {
    width_boundary: f32,
    height_boundary: f32,
}

pub struct GameData {
    window: WindowData,
    score: i16,
}

impl GameData {
    fn new(width: f32, height: f32) -> Self {
        GameData {
            window: WindowData {
                width_boundary: width / 2.0,
                height_boundary: height / 2.0,
            },
            score: 0,
        }
    }
}


 #[derive(Clone, Deserialize, Debug)]
pub struct InitializeData {
    width: f32,
    height: f32,
    title: String,
    resizable: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AssetInformation {
    sprite_image: String,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    scale: f32,
}

fn main() {
    let init_data = load_init_data();

    App::new()
        .insert_resource(WindowDescriptor {
            width: init_data.width,
            height: init_data.height,
            title: init_data.title,
            mode: WindowMode::Windowed,
            resizable: init_data.resizable,
            ..Default::default()
        })
        .insert_resource(GameData::new(init_data.width, init_data.height))
        .add_startup_system(system_setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShipPlugin)
        .add_plugin(AsteroidPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .run();
}


fn load_init_data() -> InitializeData {
    let file = File::open("resources/init_data.ron").expect("Failed opening file");
    from_reader(file).expect("Unable to load initialization data")
}

fn system_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}