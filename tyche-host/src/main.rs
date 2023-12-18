#![allow(clippy::type_complexity)]
use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{renet::*, *};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(RenetServerPlugin)
        .insert_resource(RenetServer::new(ConnectionConfig::default()))
        .add_systems(Update, handle_events_system)
        .add_systems(Update, receive_message_system)
        .run();
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            // Handle received message
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
