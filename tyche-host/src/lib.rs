use std::collections::HashMap;

use bevy::{prelude::*, utils::Uuid};
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Component, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub name: String,
    pub color: Color,
    pub portrait: Option<String>,
}


#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Option<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMovement(pub Uuid, pub PlayerInput);

#[derive(Debug, Event, Serialize, Deserialize)]
pub struct SpawnToken(pub Token);

pub fn spawn_token(commands: &mut Commands, token: &Token) {
    commands.spawn((token.clone(), Transform::default(), PlayerInput::default()));
}
