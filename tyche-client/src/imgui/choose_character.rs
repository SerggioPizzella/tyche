use bevy::{
    core::Name,
    ecs::{
        event::EventWriter,
        schedule::NextState,
        system::{Res, ResMut},
    },
};
use bevy_egui::{egui::Window, EguiContexts};
use reqwest::StatusCode;

use crate::{character_service, user::User, SpawnToken, Token};

use super::GameMenus;

#[derive(Default)]
pub struct ChooseCharacterWindow {}

pub fn load_characters(mut user: ResMut<User>, mut menu_state: ResMut<NextState<GameMenus>>) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(character_service!())
        .bearer_auth(&user.token)
        .send();


    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                user.characters = response.json().unwrap();
                menu_state.set(GameMenus::ChooseCharacter);
            }
        },
        Err(_) => menu_state.set(GameMenus::Failed),
    }

}

pub fn choose_character_ui(
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
