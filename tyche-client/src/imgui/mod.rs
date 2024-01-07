use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs, States},
        system::Local,
    },
};
use bevy_egui::{egui::Window, EguiContexts};
use serde::{Deserialize, Serialize};

use self::choose_character::choose_character_ui;

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMenus>().add_systems(
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
    ChooseCharacter,
}

mod choose_character {
    use bevy::ecs::system::Local;
    use bevy_egui::{egui::Window, EguiContexts};

    use super::Character;

    #[derive(Default)]
    pub(super) struct ChooseCharacterWindow {
        characters: Vec<Character>,
    }

    pub(super) fn choose_character_ui(
        mut contexts: EguiContexts,
        mut ui_state: Local<ChooseCharacterWindow>,
    ) {
        ui_state.characters = vec![Character::new("dog".into()), Character::new("cat".into())];

        Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Choose your character: ");
                for character in &ui_state.characters {
                    let _ = ui.button(&character.name);
                }
            });
        });
    }
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

#[derive(Default)]
struct CreateCharacterWindow {
    character_name: String,
}

fn create_character_ui(mut contexts: EguiContexts, mut ui_state: Local<CreateCharacterWindow>) {
    Window::new("Hello").show(contexts.ctx_mut(), |ui| {
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
