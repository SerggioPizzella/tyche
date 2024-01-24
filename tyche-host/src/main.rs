use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{input::common_conditions::input_toggle_active, prelude::*, utils::Uuid};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::{
    renet::{transport::*, *},
    transport::NetcodeServerPlugin,
    *,
};
use tyche_host::*;

const PLAYER_MOVE_SPEED: f32 = 200.0;

fn main() {
    let (server, transport) = new_renet_server();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(server)
        .insert_resource(transport)
        .insert_resource(Lobby::default())
        .add_systems(
            Update,
            (
                server_update_system,
                server_sync_players,
                move_players_system,
            ),
        )
        .run();
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}

fn server_update_system(
    mut lobby: ResMut<Lobby>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
    mut tokens: Query<(&Token, &mut PlayerInput)>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                lobby.players.insert(*client_id, None);
                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();

                server.broadcast_message_except(
                    *client_id,
                    DefaultChannel::ReliableUnordered,
                    message,
                );
            }

            ServerEvent::ClientDisconnected { client_id, .. } => {
                lobby.players.remove(client_id);
                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();

                server.broadcast_message_except(
                    *client_id,
                    DefaultChannel::ReliableUnordered,
                    message,
                );
            }
        }
    }

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            if let Ok(spawn_token_event) = bincode::deserialize::<SpawnToken>(&message) {
                println!("spawn token event");
                spawn_token(&mut commands, &spawn_token_event.0);

                server.broadcast_message_except(
                    client_id,
                    DefaultChannel::ReliableUnordered,
                    message.clone(),
                );
            };

            if let Ok(token_movement) = bincode::deserialize::<TokenMovement>(&message) {
                for (token, mut player_input) in tokens.iter_mut() {
                    if token.id == token_movement.0 {
                        *player_input = token_movement.1;
                    }
                }
            };
        }
    }
}

fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Token, &Transform)>) {
    let token_positions: HashMap<Uuid, [f32; 3]> = query
        .iter()
        .map(|(token, transform)| (token.id, transform.translation.into()))
        .collect();

    let sync_message = bincode::serialize(&token_positions).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);
}

fn move_players_system(mut query: Query<(&mut Transform, &PlayerInput)>, time: Res<Time>) {
    for (mut transform, input) in query.iter_mut() {
        let x = (input.right as i8 - input.left as i8) as f32;
        let y = (input.up as i8 - input.down as i8) as f32;
        transform.translation.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
        transform.translation.y += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
    }
}
