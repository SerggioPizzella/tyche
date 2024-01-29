#![allow(clippy::type_complexity)]
use clap::Parser;
use tyche_client::{bevy_world, config::Config, bevy_async};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize the logger.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::ERROR)
        .init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // Create 2 way channels for communication between bevy and tokio.
    // This is needed because we want to use the tokio runtime to make
    // requests to our services, but we also want to use bevy to render
    // our UI and run the game world.
    let (tokio_tx, bevy_rx) = tokio::sync::mpsc::channel(100);
    let (bevy_tx, tokio_rx) = tokio::sync::mpsc::channel(100);

    {
        let config = config.clone();
        // Run the tokio app in the main thread.
        // This will block until the app exits.
        tokio::spawn(async {
            bevy_async::handle_messages(config, tokio_tx, tokio_rx).await;
        });
    }

    bevy_world::start(config, bevy_tx, bevy_rx);
}
