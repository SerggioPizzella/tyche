use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Default, Resource)]
pub struct User {
    pub name: String,
    pub token: String,
    pub characters: Vec<Character>,
}
