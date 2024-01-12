use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

use super::GameMenus;

pub struct ChooseServerUI;

impl Plugin for ChooseServerUI {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<ChooseServerState>()
            .add_systems(OnEnter(GameMenus::ChooseCharacter), setup)
            .add_systems(OnEnter(ChooseServerState::Loading), load_characters)
            .add_systems(
                Update,
                choose_character_ui.run_if(in_state(ChooseServerState::Loaded)),
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum ChooseServerState {
    #[default]
    Disabled,
    Loading,
    Loaded,
}

fn setup(mut state: ResMut<NextState<ChooseServerState>>) {
    state.set(ChooseServerState::Loading);
}

fn load_characters(
    _state: ResMut<NextState<ChooseServerState>>,
    _menu_state: ResMut<NextState<GameMenus>>,
) {
}

fn choose_character_ui(
    mut contexts: EguiContexts,
    _state: ResMut<NextState<ChooseServerState>>,
    _menu_state: ResMut<NextState<GameMenus>>,
) {
    Window::new("Choose your character").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|_ui| {
        });
    });
}
