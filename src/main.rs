// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod camera2d;
mod loading;
mod player;
mod tilemap;

use crate::camera2d::Camera2DPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

// Main state of the game
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Preparing, // Game preparation state
    Running,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_plugins(Camera2DPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
