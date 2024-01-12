use std::{net::UdpSocket, time::SystemTime};

use bevy::{prelude::*, utils::HashSet};
use bevy_renet::{
    renet::{transport::*, *},
    transport::NetcodeServerPlugin,
    *,
};
use tyche_host::ServerMessages;

fn main() {
    let (server, transport) = new_renet_server();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
        .insert_resource(server)
        .insert_resource(transport)
        .insert_resource(Lobby::default())
        .add_systems(Update, server_update_system)
        .run();
}

#[derive(Debug, Default, Resource)]
struct Lobby {
    players: HashSet<ClientId>,
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
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                lobby.players.insert(*client_id);
                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message_except(
                    *client_id,
                    DefaultChannel::ReliableUnordered,
                    message,
                );
            }

            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
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
}
