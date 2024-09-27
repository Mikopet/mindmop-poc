use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::AppState;

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .db
        .execute("SELECT COUNT(*) FROM graph LIMIT 0", ())
        .await
    {
        Ok(_) => (StatusCode::OK, String::from("ok")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
