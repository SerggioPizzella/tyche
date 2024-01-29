use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    utils::Uuid,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::{
    renet::{transport::*, *},
    transport::NetcodeClientPlugin,
    *,
};
use tokio::sync::mpsc::{Sender, Receiver};
use tyche_host::PlayerInput;

use crate::{config::Config, imgui::*, menu::MenuPlugin, token::*, user::User, BevyMessage, TokioMessage};

pub fn start(
    config: Config,
    bevy_tx: Sender<BevyMessage>,
    bevy_rx: Receiver<TokioMessage>,
) {
    App::new()
        .add_state::<GameState>()
        .insert_resource(User::default())
        .insert_resource(PlayerInput::default())
        .insert_resource(config)
        .insert_resource(ChannelSender(bevy_tx))
        .insert_resource(ChannelReceiver(bevy_rx))
        .add_event::<ConnectToServer>()
        .add_plugins((DefaultPlugins, MenuPlugin, ImguiPlugin, TokenPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        // Renet
        .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
        .insert_resource(new_renet_transport())
        .add_systems(
            Update,
            (
                receive_message_system.run_if(client_connected()),
                client_send_input.run_if(client_connected()),
                sync_players.run_if(client_connected()),
            ),
        )
        .add_systems(Startup, (start_setup, send_message))
        .add_systems(Update, (player_input, connect_to_server, receive_response))
        .add_systems(OnEnter(GameState::Main), start_imgui)
        .run();
}

#[derive(Clone, Debug, Deref, Resource)]
pub struct ChannelSender(Sender<BevyMessage>);

#[derive(Debug, Deref, DerefMut, Resource)]
struct ChannelReceiver(Receiver<TokioMessage>);

fn send_message(sender: Res<ChannelSender>) {
    println!("Bevy::Sending message");
    sender.try_send(BevyMessage::Ping).unwrap();
}

fn receive_response(mut receiver: ResMut<ChannelReceiver>) {
    while let Ok(message) = receiver.try_recv() {
        match message {
            TokioMessage::FetchedUser(user) => {
                if user.is_some() {
                    println!("Bevy::Fetched user");
                }
            }
            TokioMessage::Ping => {
                println!("Bevy::Received Ping");
            }
        }
    }
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

#[derive(Clone, Debug, Event)]
pub struct ConnectToServer(pub String);

fn connect_to_server(mut commands: Commands, mut ev_connect: EventReader<ConnectToServer>) {
    for _ in ev_connect.read() {
        commands.insert_resource(RenetClient::new(ConnectionConfig::default()));
    }
}

fn receive_message_system(mut commands: Commands, mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableUnordered) {
        if let Ok(server_message) = bincode::deserialize::<ServerMessages>(&message) {
            match server_message {
                ServerMessages::PlayerConnected { id } => {
                    println!("Player {} connected.", id);
                }
                ServerMessages::PlayerDisconnected { .. } => {}
            }
        }
        if let Ok(spawn_player_event) = bincode::deserialize::<SpawnToken>(&message) {
            print!("Spawn player event");
            spawn_token(&mut commands, &spawn_player_event.0);
        }
    }
}

fn start_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn start_imgui(mut menu_state: ResMut<NextState<GameMenus>>) {
    menu_state.set(GameMenus::ChooseCharacter);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Main,
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    player_input.right =
        keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
    player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
}

fn client_send_input(
    my_token: Res<MyToken>,
    player_input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>,
) {
    let Some(token_id) = my_token.0 else {
        return;
    };

    let movement = TokenMovement(token_id, *player_input);
    let input_message = bincode::serialize(&movement).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

fn sync_players(
    mut client: ResMut<RenetClient>,
    mut player_tokens: Query<(&Token, &mut Transform)>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<Uuid, [f32; 3]> = bincode::deserialize(&message).unwrap();

        for (player_token, mut transform) in player_tokens.iter_mut() {
            if let Some(player_position) = players.get(&player_token.id) {
                transform.translation.x = player_position[0];
                transform.translation.y = player_position[1];
                transform.translation.z = player_position[2];
            }
        }
    }
}
