use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, States},
        system::{Res, ResMut, Resource},
    },
};
use bevy_egui::{egui::Window, EguiContexts};

use crate::{
    character_service,
    user::{Character, User},
};

mod choose_character;
use choose_character::{choose_character_ui, load_characters};

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMenus>()
            .insert_resource(CreateCharacterWindow::default())
            .add_systems(OnEnter(GameMenus::LoadCharacters), load_characters)
            .add_systems(
                OnEnter(GameMenus::CreateCharacter),
                on_enter_create_character_ui,
            )
            .add_systems(
                Update,
                (
                    create_character_ui.run_if(in_state(GameMenus::CreateCharacter)),
                    choose_character_ui.run_if(in_state(GameMenus::ChooseCharacter)),
                    failed_ui.run_if(in_state(GameMenus::Failed)),
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
    Failed,
}

#[derive(Default, Resource)]
struct CreateCharacterWindow {
    character_name: String,
}

fn on_enter_create_character_ui(mut ui_state: ResMut<CreateCharacterWindow>) {
    ui_state.character_name = "".to_owned();
}

fn failed_ui(mut ctx: EguiContexts, mut menu_state: ResMut<NextState<GameMenus>>) {
    Window::new("Something went wrong").show(ctx.ctx_mut(), |ui| {
        if ui.button("Reload Characters").clicked() {
            menu_state.set(GameMenus::LoadCharacters);
        }
    });
}

fn create_character_ui(
    user: Res<User>,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<CreateCharacterWindow>,
    mut menu_state: ResMut<NextState<GameMenus>>,
) {
    Window::new("Create character".to_owned() + &user.name).show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Write a name: ");
            ui.text_edit_singleline(&mut ui_state.character_name);
        });

        ui.horizontal(|ui| {
            if ui.small_button("<").clicked() {
                menu_state.set(GameMenus::ChooseCharacter);
            }
            if ui.button("Create").clicked() {
                let client = reqwest::blocking::Client::new();
                let _reply = client
                    .post(character_service!())
                    .header("Content-Type", "application/json")
                    .json(&Character::new(ui_state.character_name.clone()))
                    .send();

                menu_state.set(GameMenus::LoadCharacters);
            }
        })
    });
}
