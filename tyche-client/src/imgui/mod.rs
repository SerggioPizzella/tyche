use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, States, NextState},
        system::{Local, Res, ResMut},
    },
};
use bevy_egui::{egui::Window, EguiContexts};

use crate::{
    character_service,
    user::{Character, User},
};

use self::choose_character::{choose_character_ui, load_characters};

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMenus>()
            .add_systems(OnEnter(GameMenus::LoadCharacters), load_characters)
            .add_systems(
                Update,
                (
                    create_character_ui.run_if(in_state(GameMenus::CreateCharacter)),
                    choose_character_ui.run_if(in_state(GameMenus::ChooseCharacter)),
                ),
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameMenus {
    #[default]
    None,
    CreateCharacter,
    LoadCharacters,
    ChooseCharacter,
}

mod choose_character {
    use bevy::ecs::{
        schedule::NextState,
        system::{Res, ResMut},
    };
    use bevy_egui::{egui::Window, EguiContexts};
    use reqwest::StatusCode;

    use crate::{character_service, user::User};

    use super::GameMenus;

    #[derive(Default)]
    pub(super) struct ChooseCharacterWindow {}

    pub(super) fn load_characters(
        mut user: ResMut<User>,
        mut menu_state: ResMut<NextState<GameMenus>>,
    ) {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(character_service!())
            .bearer_auth(&user.token)
            .send()
            .unwrap();

        if response.status() == StatusCode::OK {
            user.characters = response.json().unwrap();
            menu_state.set(GameMenus::ChooseCharacter);
        }
    }

    pub(super) fn choose_character_ui(
        mut contexts: EguiContexts,
        user: Res<User>,
        mut menu_state: ResMut<NextState<GameMenus>>,
    ) {
        Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Choose your character: ");
                for character in &user.characters {
                    let _ = ui.button(&character.name);
                }

                if ui.button("Create new character").clicked() {
                    menu_state.set(GameMenus::CreateCharacter);
                }
            });
        });
    }
}

#[derive(Default)]
struct CreateCharacterWindow {
    character_name: String,
}

fn create_character_ui(
    user: Res<User>,
    mut contexts: EguiContexts,
    mut ui_state: Local<CreateCharacterWindow>,
    mut menu_state: ResMut<NextState<GameMenus>>,
) {
    Window::new("Hello ".to_owned() + &user.name).show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Write a name: ");
            ui.text_edit_singleline(&mut ui_state.character_name);
        });

        if ui.button("Create character").clicked() {
            let client = reqwest::blocking::Client::new();
            let _reply = client
                .post(character_service!())
                .header("Content-Type", "application/json")
                .json(&Character::new(ui_state.character_name.clone()))
                .send();

            menu_state.set(GameMenus::LoadCharacters);
        }
    });
}
