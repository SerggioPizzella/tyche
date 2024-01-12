use bevy::{
    app::{Plugin, Update},
    core::Name,
    ecs::{
        event::EventWriter,
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, States, OnExit},
        system::{Res, ResMut},
    },
};
use bevy_egui::{egui::Window, EguiContexts};
use reqwest::StatusCode;

use crate::{
    character_service,
    token::{SpawnToken, Token},
    user::User,
};

use super::GameMenus;

pub struct ChooseCharacterUI;

impl Plugin for ChooseCharacterUI {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<ChooseCharacterState>()
            .add_systems(OnEnter(GameMenus::ChooseCharacter), setup)
            .add_systems(OnExit(GameMenus::ChooseCharacter), exit)
            .add_systems(OnEnter(ChooseCharacterState::Loading), load_characters)
            .add_systems(
                Update,
                choose_character_ui.run_if(in_state(ChooseCharacterState::Loaded)),
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum ChooseCharacterState {
    #[default]
    Disabled,
    Loading,
    Loaded,
}

fn setup(mut state: ResMut<NextState<ChooseCharacterState>>) {
    state.set(ChooseCharacterState::Loading);
}

fn exit(mut state: ResMut<NextState<ChooseCharacterState>>) {
    state.set(ChooseCharacterState::Disabled);
}

fn load_characters(
    mut user: ResMut<User>,
    mut state: ResMut<NextState<ChooseCharacterState>>,
    mut menu_state: ResMut<NextState<GameMenus>>,
) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(character_service!())
        .bearer_auth(&user.token)
        .send();

    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                user.characters = response.json().unwrap();
                state.set(ChooseCharacterState::Loaded);
            }
        }
        Err(_) => {
            menu_state.set(GameMenus::Failed);
        }
    }
}

fn choose_character_ui(
    user: Res<User>,
    mut contexts: EguiContexts,
    mut ev_spawn_token: EventWriter<SpawnToken>,
    mut menu_state: ResMut<NextState<GameMenus>>,
) {
    Window::new("Choose your character").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            for character in &user.characters {
                if ui.button(&character.name).clicked() {
                    ev_spawn_token.send(SpawnToken(Token::new(Name::new(character.name.clone()))));
                }
            }

            if ui.button("Create new character").clicked() {
                menu_state.set(GameMenus::CreateCharacter);
            }
        });
    });
}
