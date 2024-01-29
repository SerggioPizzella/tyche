use bevy::{prelude::*, utils::Uuid};
use bevy_egui::{
    egui::{Color32, Rgba, Window},
    EguiContexts,
};
use bevy_renet::renet::RenetClient;
use reqwest::StatusCode;

use crate::{
    config::{self, Config},
    token::{MyToken, SpawnToken, Token},
    user::User,
};

use super::{CurrentCharacter, GameMenus};

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
    config: Res<Config>,
    mut user: ResMut<User>,
    mut state: ResMut<NextState<ChooseCharacterState>>,
    mut menu_state: ResMut<NextState<GameMenus>>,
) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&config.character_service)
        .bearer_auth(&user.fire_token)
        .send();

    match response {
        Ok(response) => {
            if response.status() != StatusCode::OK {
                tracing::error!("Error: {:?}", response);
                menu_state.set(GameMenus::Failed);
                return;
            }

            user.characters = response.json().unwrap();
            state.set(ChooseCharacterState::Loaded);
        }
        Err(e) => {
            println!("Error: {}", e);
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
    mut current_character: ResMut<CurrentCharacter>,
    client: Option<Res<RenetClient>>,
) {
    Window::new("Choose your character").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            for character in &user.characters {
                let color = &character.color;
                ui.horizontal(|ui| {
                    ui.colored_label(
                        Color32::from_rgba_unmultiplied(
                            color.red,
                            color.green,
                            color.blue,
                            color.alpha,
                        ),
                        &character.name,
                    );

                    if ui.small_button("change color").clicked() {
                        current_character.0 = Some(character.clone());
                        menu_state.set(GameMenus::ChangeColor);
                    }

                    if client.is_some() {
                        if ui.button("spawn").clicked() {
                            let uuid = Uuid::new_v4();

                            my_token.0 = Some(uuid);
                            ew_spawn_token.send(SpawnToken(Token {
                                id: uuid,
                                name: character.name.clone(),
                                portrait: character.portrait.clone(),
                                color: Color::Rgba {
                                    red: character.color.red as f32,
                                    green: character.color.green as f32,
                                    blue: character.color.blue as f32,
                                    alpha: character.color.alpha as f32,
                                },
                            }));
                        }
                    }
                });
            }

            if ui.button("Create new character").clicked() {
                menu_state.set(GameMenus::CreateCharacter);
            }
        });
    });
}
