use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};
use bevy_renet::renet::RenetClient;

use crate::bevy_world::ConnectToServer;

pub struct ChooseServerUI<T>
where
    T: States,
{
    state: T,
}

impl<T> ChooseServerUI<T>
where
    T: States,
{
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for ChooseServerUI<T>
where
    T: States,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<ChooseServerState>()
            .add_systems(OnEnter(self.state.clone()), setup)
            .add_systems(OnExit(self.state.clone()), exit)
            .add_systems(
                Update,
                choose_server_ui.run_if(in_state(ChooseServerState::Main)),
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum ChooseServerState {
    #[default]
    Disabled,
    Main,
}

fn setup(mut state: ResMut<NextState<ChooseServerState>>) {
    state.set(ChooseServerState::Main);
}

fn exit(mut state: ResMut<NextState<ChooseServerState>>) {
    state.set(ChooseServerState::Disabled);
}

#[derive(Default)]
struct ChooseServerUi {
    server_address: String,
}

fn choose_server_ui(
    mut contexts: EguiContexts,
    mut choose_server_ui: Local<ChooseServerUi>,
    mut ev_connect: EventWriter<ConnectToServer>,
    mut client: Option<ResMut<RenetClient>>,
) {
    Window::new("Connect to server").show(contexts.ctx_mut(), |ui| {
        // button to disconnect from server
        if let Some(client) = client.as_mut() {
            if ui.button("Disconnect").clicked() {
                client.disconnect();
            }
            return;
        }

        // input field for server address and port
        ui.horizontal(|ui| {
            ui.label("Server address:");
            ui.text_edit_singleline(&mut choose_server_ui.server_address);
        });

        // button to connect to server
        if ui.button("Connect").clicked() {
            ev_connect.send(ConnectToServer(choose_server_ui.server_address.clone()));
        }
    });
}
