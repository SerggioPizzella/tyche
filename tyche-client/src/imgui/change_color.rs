use bevy::prelude::*;
use bevy_egui::{
    egui::{Color32, Window},
    EguiContexts,
};

use crate::{bevy_world::ChannelSender, user::User, BevyMessage};

use super::{CurrentCharacter, GameMenus};

pub struct ChooseColorUI<T>
where
    T: States,
{
    state: T,
}

impl<T> ChooseColorUI<T>
where
    T: States,
{
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for ChooseColorUI<T>
where
    T: States,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(self.state.clone()), setup)
            .add_systems(OnExit(self.state.clone()), exit)
            .add_systems(Update, change_color_ui.run_if(in_state(MenuState::Main)));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Disabled,
    Main,
}

fn setup(mut state: ResMut<NextState<MenuState>>) {
    state.set(MenuState::Main);
}

fn exit(mut state: ResMut<NextState<MenuState>>) {
    state.set(MenuState::Disabled);
}

fn change_color_ui(
    user: Res<User>,
    sender: ResMut<ChannelSender>,
    mut contexts: EguiContexts,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut windows_state: ResMut<NextState<GameMenus>>,
    mut current_character: ResMut<CurrentCharacter>,
    mut color: Local<Color32>,
) {
    Window::new("Choose a color").show(contexts.ctx_mut(), |ui| {
        ui.color_edit_button_srgba(&mut color);
        if ui.small_button("Choose").clicked() {
            let _ = sender.try_send(BevyMessage::PatchCharacterColor {
                token: user.fire_token.clone(),
                id: current_character.0.as_ref().clone().unwrap().id.unwrap(),
                color: [color.r(), color.g(), color.b(), color.a()],
            });

            current_character.0 = None;
            menu_state.set(MenuState::Disabled);
            windows_state.set(GameMenus::ChooseCharacter);
        }
    });
}
