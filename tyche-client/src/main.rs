#![allow(clippy::type_complexity)]
mod menu;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, menu::MenuPlugin))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
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

#[derive(Debug, Deserialize, Serialize)]
struct Character {
    name: String,
}

impl Character {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn ui_example_system(mut contexts: EguiContexts, mut ui_state: Local<String>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Write a name: ");
            ui.text_edit_singleline(&mut *ui_state);
        });

        if ui.button("Create character").clicked() {
            let client = reqwest::blocking::Client::new();
            _ = client
                .post("localhost:8000/")
                .header("Content-Type", "application/json")
                .json(&Character::new((*ui_state).clone()))
                .send();
        }
    });
}
