use std::net::SocketAddr;

use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::{get, patch}, BoxError, Router};
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use sqlx::MySqlPool;
use tower::{buffer::BufferLayer, ServiceBuilder};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use crate::{config::Config, http::state::AppState};

use self::character::*;

mod character;
mod state;

pub async fn serve(config: Config, db: MySqlPool) -> anyhow::Result<()> {
    info!("Starting server on port {}", &config.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .context("error binding TCP listener")?;

    let governor_config = GovernorConfigBuilder::default()
        .per_second(4)
        .burst_size(2)
        .finish()
        .unwrap();

    let governor_config = Box::new(governor_config);

    let firebase_auth = FirebaseAuth::new("tyche-vtt").await;

    let app = api_router()
        .with_state(AppState {
            pool: db,
            firebase: FirebaseAuthState { firebase_auth },
        })
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    error!("Unhandled error: {}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1))
                .layer(GovernorLayer {
                    config: Box::leak(governor_config),
                }),
        );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("error running HTTP server")
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/v1", get(get_characters).post(create_character))
        .route("/v1/:id", patch(patch_character_color))
}
