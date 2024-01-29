use firebase_auth::FirebaseUser;
pub use tyche_character::Character;

pub mod config;
mod imgui;
pub mod menu;
pub mod token;

pub mod bevy_async;
pub mod bevy_world;
pub mod firebase;
pub mod user;

#[derive(Clone, Debug)]
pub enum BevyMessage {
    Ping,
    CreateCharacter(String, Character),
    FetchToken(String),
    PatchCharacterColor { token: String, id: i32, color: [u8; 4] },
}

#[derive(Clone)]
pub enum TokioMessage {
    Ping,
    FetchedUser(Option<FirebaseUser>),
}
