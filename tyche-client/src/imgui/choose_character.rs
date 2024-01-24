use bevy::{prelude::*, utils::Uuid};
use bevy_egui::{egui::Window, EguiContexts};
use bevy_renet::renet::RenetClient;
use reqwest::StatusCode;

use crate::{
    config,
    token::{SpawnToken, Token, MyToken},
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
        .get(config::character_service())
        .bearer_auth(&user.fire_token)
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
    mut my_token: ResMut<MyToken>,
    mut contexts: EguiContexts,
    mut ew_spawn_token: EventWriter<SpawnToken>,
    mut menu_state: ResMut<NextState<GameMenus>>,
    client: Option<Res<RenetClient>>,
) {
    Window::new("Choose your character").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            for character in &user.characters {
                if client.is_some() {
                    if ui.button(&character.name).clicked() {
                        let uuid = Uuid::new_v4();

                        my_token.0 = Some(uuid);
                        ew_spawn_token.send(SpawnToken(Token {
                            id: uuid,
                            name: character.name.clone(),
                            portrait: character.portrait.clone(),
                            color: Color::Rgba {
                                red: character.color.red,
                                green: character.color.green,
                                blue: character.color.blue,
                                alpha: character.color.alpha,
                            },
                        }));
                    }
                } else {
                    ui.label(&character.name);
                }
            }

            if ui.button("Create new character").clicked() {
                menu_state.set(GameMenus::CreateCharacter);
            }
        });
    });
}
