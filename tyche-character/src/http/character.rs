use axum::{
    extract::{Path, Query, State},
    Json,
};
use firebase_auth::FirebaseUser;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{Character, Color};

pub async fn create_character(
    _user: FirebaseUser,
    State(pool): State<MySqlPool>,
    Json(character): Json<Character>,
) {
    let query = sqlx::query!(
        "INSERT INTO character.character (name, color, owner) VALUES (?, ?, ?)",
        character.name,
        character.color.to_string(),
        character.owner
    );

    query.execute(&pool).await.unwrap();
}

pub async fn patch_character_color(
    _user: FirebaseUser,
    Path(character_id): Path<u32>,
    State(pool): State<MySqlPool>,
    Json(color): Json<Color>,
) {
    let query = sqlx::query!(
        "UPDATE character.character SET color = ? WHERE id = ?",
        color.to_string(),
        character_id
    );

    query.execute(&pool).await.unwrap();
}

#[derive(Deserialize, Default)]
pub enum CharacterFilter {
    #[default]
    All,
    Mine,
}

pub async fn get_characters(
    _user: FirebaseUser,
    State(pool): State<MySqlPool>,
) -> Json<Vec<Character>> {
    get_all_characters(pool).await
}

pub async fn get_my_characters(pool: MySqlPool, user_id: &str) -> Json<Vec<Character>> {
    let characters = sqlx::query_as!(
        Character,
        "SELECT * FROM character.character WHERE owner = ?",
        user_id
    )
    .fetch_all(&pool)
    .await;

    if let Err(e) = characters {
        tracing::error!("Error fetching characters: {:?}", e);
        return Json(vec![]);
    }

    Json(characters.unwrap())
}

pub async fn get_all_characters(pool: MySqlPool) -> Json<Vec<Character>> {
    let characters = sqlx::query_as!(Character, "SELECT * FROM character.character")
        .fetch_all(&pool)
        .await;

    if let Err(e) = characters {
        tracing::error!("Error fetching characters: {:?}", e);
        return Json(vec![]);
    }

    Json(characters.unwrap())
}
