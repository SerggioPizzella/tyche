use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, States},
        system::{ResMut, Resource},
    },
};
use bevy_egui::{egui::Window, EguiContexts};

use choose_character::ChooseCharacterUI;
use create_character::CreateCharacterUI;
use tyche_character::Character;

use crate::bevy_world::GameState;

use self::{change_color::ChooseColorUI, choose_server::ChooseServerUI};

mod choose_character;
mod change_color;
mod create_character;
mod choose_server;

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMenus>()
            .insert_resource(CurrentCharacter(None))
            .add_plugins((
                ChooseCharacterUI,
                CreateCharacterUI,
                ChooseServerUI::new(GameState::Main),
                ChooseColorUI::new(GameMenus::ChangeColor),
            ))
            .add_systems(Update, failed_ui.run_if(in_state(GameMenus::Failed)));
    }
}

#[derive(Resource)]
pub struct CurrentCharacter(pub Option<Character>);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameMenus {
    #[default]
    None,
    ChangeColor,
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
