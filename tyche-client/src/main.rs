#![allow(clippy::type_complexity)]
mod menu;

use bevy::{prelude::*, tasks::Task};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use firebase_auth::FirebaseAuth;
use futures_lite::future;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let firebase_auth = FirebaseAuth::new("tyche-cloud").await;

    App::new()
        .insert_resource(Fire(firebase_auth))
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, menu::MenuPlugin))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, start_setup)
        .add_systems(Update, complete_setup.run_if(in_state(GameState::Setup)))
        .add_systems(Update, ui_example_system)
        .run();
}

#[derive(Component)]
struct GenericTask<T>(Task<T>);

fn start_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn complete_setup(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut tasks: Query<(Entity, &mut GenericTask<FirebaseAuth>)>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(firebase_auth) = future::block_on(future::poll_once(&mut task.0)) {
            commands.insert_resource(Fire(firebase_auth));
            commands.entity(entity).despawn();
            game_state.set(GameState::Menu);
        }
    }
}

#[derive(Resource)]
struct Fire(FirebaseAuth);

#[derive(Resource)]
struct User;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Setup,
    Menu,
}

mod host {
    // add code here
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
