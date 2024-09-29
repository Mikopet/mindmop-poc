use axum::extract::State;

use crate::{graph::Stats, AppState};

use super::types::JsonResponse;

pub async fn health(State(state): State<AppState>) -> JsonResponse<Stats> {
    Stats::get(&state.db).await.into()
}
