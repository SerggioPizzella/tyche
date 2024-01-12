#![allow(clippy::type_complexity)]
mod firebase;
mod imgui;
mod menu;
mod user;

use std::{net::UdpSocket, time::SystemTime};

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::{
    renet::{transport::*, *},
    transport::NetcodeClientPlugin,
    *,
};
use dotenvy::dotenv;
use imgui::{GameMenus, ImguiPlugin};

use menu::MenuPlugin;
use tyche_host::ServerMessages;
use user::User;

fn main() {
    let _ = dotenv();

    App::new()
        .add_state::<GameState>()
        .add_event::<SpawnToken>()
        .insert_resource(User::default())
        .add_plugins((DefaultPlugins, MenuPlugin, ImguiPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        // Renet
        .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
        //.insert_resource(client)
        .insert_resource(new_renet_transport())
        .add_systems(
            Update,
            receive_message_system.run_if(resource_exists::<RenetClient>()),
        )
        .add_systems(Startup, start_setup)
        .add_systems(OnEnter(GameState::Main), (start_imgui, connect_to_server))
        .add_systems(Update, handle_spawn_token)
        .run();
}
fn new_renet_transport() -> NetcodeClientTransport {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };

    NetcodeClientTransport::new(current_time, authentication, socket).unwrap()
}

fn connect_to_server(mut commands: Commands) {
    commands.insert_resource(RenetClient::new(ConnectionConfig::default()));
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(_) = client.receive_message(DefaultChannel::ReliableOrdered) {
        print!("Got reliable message");
    }
    while let Some(message) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
            }
            ServerMessages::PlayerDisconnected { .. } => {}
        }
    }
    while let Some(_) = client.receive_message(DefaultChannel::Unreliable) {
        print!("got unreliable message");
    }
}

#[macro_export]
macro_rules! auth_service {
    () => {
        std::env::var("AUTH_SERVICE").unwrap()
    };
}
#[macro_export]
macro_rules! character_service {
    () => {
        std::env::var("CHARACTER_SERVICE").unwrap()
    };
}

#[derive(Component, Clone)]
struct Token {
    name: Name,
}

impl Token {
    fn new(name: Name) -> Self {
        Self { name }
    }
}

#[derive(Bundle)]
struct TokenBundle {
    name: Name,
    token: Token,
    button: ButtonBundle,
}

#[derive(Event)]
struct SpawnToken(Token);

fn handle_spawn_token(
    mut ev_spawn_token: EventReader<SpawnToken>,
    tokens: Query<Entity, With<Token>>,
    mut commands: Commands,
) {
    for event in ev_spawn_token.read() {
        for token in &tokens {
            commands.entity(token).despawn_recursive();
        }

        commands
            .spawn(TokenBundle {
                token: event.0.clone(),
                name: event.0.name.clone(),
                button: ButtonBundle {
                    background_color: Color::rgb(0.8, 0.15, 0.15).into(),
                    style: Style {
                        padding: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                    ..default()
                },
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    &event.0.name,
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                ));
            });
    }
}

fn start_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn start_imgui(mut menu_state: ResMut<NextState<GameMenus>>) {
    menu_state.set(GameMenus::LoadCharacters);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Main,
}
