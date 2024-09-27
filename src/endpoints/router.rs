use axum::{routing::get, Router};

use crate::{endpoints::*, AppState};

// Router getter
pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(ui_router())
        .nest("/api", api_router(state))
}

// Router for static content
fn ui_router() -> Router {
    Router::new()
        // Health check
        .route("/health", get(ui::health))
}

// Router with DB access
fn api_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(api::health))
        // Pass AppState
        .with_state(state)
}
