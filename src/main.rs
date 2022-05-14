use std::fs::File;
use bevy::{
    prelude::*,
    window::WindowMode,
};
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
    Exit,
}

 #[derive(Clone, Deserialize, Debug)]
pub struct InitializeData {
    width: f32,
    height: f32,
    title: String,
    resizable: bool,
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
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .run();
}


fn load_init_data() -> InitializeData {
    let file = File::open("resources/init_data.ron").expect("Failed opening file");
    from_reader(file).expect("Unable to load initialization data")
}