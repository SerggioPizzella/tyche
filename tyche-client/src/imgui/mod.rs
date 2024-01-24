use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, States},
        system::ResMut,
    },
};
use bevy_egui::{egui::Window, EguiContexts};

use choose_character::ChooseCharacterUI;
use create_character::CreateCharacterUI;

use crate::GameState;

use self::choose_server::ChooseServerUI;

mod choose_character;
mod choose_server;
mod create_character;

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMenus>()
            .add_plugins((
                ChooseCharacterUI,
                CreateCharacterUI,
                ChooseServerUI::new(GameState::Main),
            ))
            .add_systems(Update, (failed_ui.run_if(in_state(GameMenus::Failed)),));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameMenus {
    #[default]
    None,
    CreateCharacter,
    ChooseCharacter,
    Failed,
}

fn failed_ui(mut ctx: EguiContexts, mut menu_state: ResMut<NextState<GameMenus>>) {
    Window::new("Something went wrong").show(ctx.ctx_mut(), |ui| {
        if ui.button("Reload Characters").clicked() {
            menu_state.set(GameMenus::ChooseCharacter);
        }
    });
}
