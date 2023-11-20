#![allow(clippy::type_complexity)]
mod menu;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, menu::MenuPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct User;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Splash,
    #[default]
    Menu,
    Game,
}
