use anyhow::Context;
use clap::Parser;

use sqlx::MySqlPool;
use tyche_character::{config::Config, http};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Initialize the logger.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = MySqlPool::connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await?;

    // Finally, we spin up our API.
    http::serve(config, db).await?;

    Ok(())
}
