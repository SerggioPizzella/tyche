pub fn auth_service() -> String {
    std::env::var("AUTH_SERVICE").unwrap()
}

pub fn character_service() -> String {
    std::env::var("CHARACTER_SERVICE").unwrap()
}

use bevy::ecs::system::Resource;
use clap::Parser;

#[derive(Clone, Parser, Resource)]
pub struct Config {
    /// The connection URL for the authentication service.
    #[clap(env)]
    pub auth_service: String,

    /// The connection URL for the character service.
    #[clap(env)]
    pub character_service: String,

    /// The connection URL for the Postgres database.
    #[clap(env)]
    pub database_url: String,

    /// The port to listen on.
    /// Defaults to 8080.
    #[clap(env, default_value = "8080")]
    pub port: u16,
}
