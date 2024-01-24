use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};
use tokio::sync::RwLock;
use tyche_character::Character;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(AppState::default()));

    let app = Router::new()
        .route("/v1", get(get_characters).post(create_character))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Default)]
struct AppState {
    characters: Vec<Character>,
}

async fn create_character(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(character): Json<Character>,
) {
    state.write().await.characters.push(character);
}

async fn get_characters(State(state): State<Arc<RwLock<AppState>>>) -> Json<Vec<Character>> {
    Json(state.read().await.characters.clone())
}
