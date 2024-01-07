use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

#[derive(Debug, Default)]
struct AppState {
    sessions: HashMap<Uuid, Option<String>>,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(AppState::default()));

    let app = Router::new()
        .route("/v1", get(generate_auth_session))
        .route("/v1/:id", post(receive_token))
        .route("/v1/:id", get(get_token))
        .with_state(shared_state);

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_auth_session(State(state): State<Arc<RwLock<AppState>>>) -> String {
    let sessions = &mut state.write().unwrap().sessions;
    let id = Uuid::new_v4();

    sessions.insert(id, None);

    id.to_string()
}

async fn receive_token(
    Path(session): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
    token: String,
) {
    state.write().unwrap().sessions.insert(session, Some(token));
}

async fn get_token(
    Path(id): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<String, StatusCode> {
    let session = &state.read().unwrap().sessions;

    if let Some(token) = session.get(&id) {
        return token.clone().ok_or(StatusCode::FOUND);
    }

    Err(StatusCode::NOT_FOUND)
}
