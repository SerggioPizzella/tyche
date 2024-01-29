use firebase_auth::FirebaseUser;
use reqwest::StatusCode;
use tokio::sync::mpsc::{Receiver, Sender};
use tyche_character::Character;

use crate::{config::Config, BevyMessage, TokioMessage};

pub async fn handle_messages(
    config: Config,
    tokio_tx: Sender<TokioMessage>,
    mut tokio_rx: Receiver<BevyMessage>,
) {
    while let Some(message) = tokio_rx.recv().await {
        match message {
            BevyMessage::CreateCharacter(fire_token, character) => {
                tracing::error!("Creating character: {:?}", character);
                create_character(&config, &fire_token, character).await;
            }
            BevyMessage::Ping => {
                println!("Tokio::Received message: {:?}", message);
                println!("Tokio::Sending message: {:?}", message);
                tokio_tx.send(TokioMessage::Ping).await.unwrap();
            }
            BevyMessage::FetchToken(session) => {
                let token = fetch_token(&config, &session).await;
                tokio_tx
                    .send(TokioMessage::FetchedUser(token))
                    .await
                    .unwrap();
            }
            BevyMessage::PatchCharacterColor { token, id, color } => {
                patch_character_color(&config, &token, id, color).await;
            }
        }
    }
}

async fn patch_character_color(config: &Config, fire_token: &str, id: i32, color: [u8; 4]) {
    let client = reqwest::Client::new();

    let reply = client
        .patch(format!("{}/{}", &config.character_service, id))
        .bearer_auth(fire_token)
        .header("Content-Type", "application/json")
        .json(&color)
        .send()
        .await;

    match reply {
        Ok(response) => {
            if response.status() != StatusCode::OK {
                tracing::error!("Error: {:?}", response);
            }
        }
        Err(_) => tracing::error!("Error sending request to create character"),
    }
}

async fn fetch_token(config: &Config, session: &str) -> Option<FirebaseUser> {
    let request = reqwest::get(format!("{}/{}", config.auth_service, session))
        .await
        .unwrap();

    if request.status() == StatusCode::OK {
        let firebase_user = request.json::<FirebaseUser>().await.unwrap();
        return Some(firebase_user);
    }

    None
}

async fn create_character(config: &Config, fire_token: &str, character: Character) {
    let client = reqwest::Client::new();

    let reply = client
        .post(&config.character_service)
        .bearer_auth(fire_token)
        .header("Content-Type", "application/json")
        .json(&character)
        .send()
        .await;

    match reply {
        Ok(response) => {
            if response.status() != StatusCode::OK {
                tracing::error!("Error: {:?}", response);
            }
        }
        Err(_) => tracing::error!("Error sending request to create character"),
    }
}
