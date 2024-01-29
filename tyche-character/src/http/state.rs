use axum_macros::FromRef;
use firebase_auth::FirebaseAuthState;
use sqlx::MySqlPool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: MySqlPool,
    pub firebase: FirebaseAuthState,
}
