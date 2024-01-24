use crate::{
    config,
    user::{Character, User},
};
use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

use super::GameMenus;
use tyche_character::Color;

pub struct CreateCharacterUI;

#[derive(Default, Resource)]
struct CreateCharacterWindow {
    character_name: String,
    character_color: [f32; 3],
    portrait: Option<String>,
}

impl Plugin for CreateCharacterUI {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<CreateCharacterState>()
            .insert_resource(CreateCharacterWindow::default())
            .add_systems(OnEnter(GameMenus::CreateCharacter), setup)
            .add_systems(OnExit(GameMenus::CreateCharacter), exit)
            .add_systems(OnEnter(CreateCharacterState::Main), init_window)
            .add_systems(
                Update,
                create_character_ui.run_if(in_state(CreateCharacterState::Main)),
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum CreateCharacterState {
    #[default]
    Disabled,
    Main,
}

fn setup(mut state: ResMut<NextState<CreateCharacterState>>) {
    state.set(CreateCharacterState::Main)
}

fn init_window(mut ui_state: ResMut<CreateCharacterWindow>) {
    ui_state.character_name = "".to_owned();
}

fn exit(mut state: ResMut<NextState<CreateCharacterState>>) {
    state.set(CreateCharacterState::Disabled);
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
            ui.label("Choose a color: ");
            ui.color_edit_button_rgb(&mut ui_state.character_color);
        });

        ui.horizontal(|ui| {
            if ui.small_button("<").clicked() {
                menu_state.set(GameMenus::ChooseCharacter);
            }

            if ui.button("Create").clicked() {
                let character = Character {
                    name: ui_state.character_name.clone(),
                    color: Color {
                        red: ui_state.character_color[0],
                        green: ui_state.character_color[1],
                        blue: ui_state.character_color[2],
                        alpha: 1.0,
                    },
                    portrait: ui_state.portrait.clone(),
                };

                let client = reqwest::blocking::Client::new();

                let _reply = client
                    .post(config::character_service())
                    .header("Content-Type", "application/json")
                    .json(&character)
                    .send();

                menu_state.set(GameMenus::ChooseCharacter);
            }
        })
    });
}
