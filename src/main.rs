// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod loading;
use crate::loading::LoadingPlugin;

// Main state of the game
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading, // Loading state
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(LoadingPlugin)
        .run();
}
