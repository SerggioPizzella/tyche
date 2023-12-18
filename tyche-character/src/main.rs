use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(AppState::default()));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_characters).post(create_character))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Default)]
struct AppState {
    characters: Vec<Character>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Character {
    name: String,
}

async fn create_character(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(character): Json<Character>,
) {
    state.write().await.characters.push(character);
}

async fn get_characters(State(state): State<Arc<RwLock<AppState>>>) -> Json<Vec<String>> {
    Json(state.read().await.characters.iter().map(|c| c.name.clone()).collect())
}
