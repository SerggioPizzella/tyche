#![allow(clippy::type_complexity)]
mod firebase;
mod menu;

use bevy::{input::common_conditions::input_toggle_active, prelude::*, tasks::Task};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dotenvy::dotenv;
use menu::MenuPlugin;
use serde::{Deserialize, Serialize};

fn main() {
    let _ = dotenv();

    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, MenuPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, start_setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn start_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct User;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
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

#[derive(Component)]
struct CreateCharacterTask(Task<()>);

#[derive(Default)]
struct CreateCharacterWindow {
    character_name: String,
}

fn ui_example_system(mut contexts: EguiContexts, mut ui_state: Local<CreateCharacterWindow>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Write a name: ");
            ui.text_edit_singleline(&mut ui_state.character_name);
        });

        if ui.button("Create character").clicked() {
            let client = reqwest::blocking::Client::new();
            let reply = client
                .post("http://localhost:3000/")
                .header("Content-Type", "application/json")
                .json(&Character::new(ui_state.character_name.clone()))
                .send();
            println!("{:?}", reply);
        }
    });
}
