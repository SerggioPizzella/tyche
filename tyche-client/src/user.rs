use bevy::prelude::*;

pub use tyche_character::Character;

#[derive(Debug, Default, Resource)]
pub struct User {
    pub name: String,
    pub fire_token: String,
    pub characters: Vec<Character>,
    pub sub: String,
}
