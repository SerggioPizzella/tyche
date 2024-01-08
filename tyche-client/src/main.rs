#![allow(clippy::type_complexity)]
mod firebase;
mod imgui;
mod menu;
mod user;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dotenvy::dotenv;
use imgui::{ImguiPlugin, GameMenus};
use menu::MenuPlugin;
use user::User;

fn main() {
    let _ = dotenv();

    App::new()
        .add_state::<GameState>()
        .insert_resource(User::default())
        .add_plugins((DefaultPlugins, MenuPlugin, ImguiPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, start_setup)
        .add_systems(OnEnter(GameState::Main), start_imgui)
        .run();
}

#[macro_export]
macro_rules! auth_service {
    () => {
        std::env::var("AUTH_SERVICE").unwrap()
    };
}
#[macro_export]
macro_rules! character_service {
    () => {
        std::env::var("CHARACTER_SERVICE").unwrap()
    };
}

fn start_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn start_imgui(mut menu_state: ResMut<NextState<GameMenus>>) {
    menu_state.set(GameMenus::LoadCharacters);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Main,
}
