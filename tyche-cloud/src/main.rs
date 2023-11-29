use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Debug, Default)]
struct AppState {
    sessions: HashMap<Uuid, Option<String>>,
}

#[derive(Debug, Deserialize)]
struct Session {
    id: Uuid,
    token: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(AppState::default()));

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/v1", get(generate_auth_session))
        .route("/v1", post(receive_token))
        .route("/v1/:id", get(get_token))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_auth_session(State(state): State<Arc<RwLock<AppState>>>) -> String {
    let sessions = &mut state.write().unwrap().sessions;
    let id = Uuid::new_v4();

    sessions.insert(id, None);

    id.to_string()
}

async fn receive_token(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(access_token): Json<Session>,
) {
    state
        .write()
        .unwrap()
        .sessions
        .insert(access_token.id, Some(access_token.token));
}

async fn get_token(
    Path(id): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<String, StatusCode> {
    let session = &state.read().unwrap().sessions;

    if let Some(token) = session.get(&id) {
        return token.clone().ok_or(StatusCode::FOUND)
    }

    Err(StatusCode::NOT_FOUND)
}
