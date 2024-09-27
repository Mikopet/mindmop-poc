mod endpoints;
mod graph;

use lazy_static::lazy_static;
use libsql::{Connection, Database};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_turso::Turso;

lazy_static! {
    static ref SEED: bool = std::env::var("SEED")
        .unwrap_or_default()
        .parse()
        .unwrap_or(false);
}

#[derive(Clone)]
struct AppState {
    db: Connection,
    secrets: SecretStore,
}

#[shuttle_runtime::main]
async fn app(
    #[Turso(
        addr = "{secrets.DB_TURSO_ADDRESS}",
        token = "{secrets.DB_TURSO_TOKEN}"
    )]
    db: Database,
    #[Secrets] secrets: SecretStore,
) -> ShuttleAxum {
    // Build DB connection
    let db = db.connect().unwrap();

    // Set up graph schema
    graph::schema::setup(&db).await?;

    // Build AppState
    let state = AppState { db, secrets };

    // Boot Axum Server
    Ok(endpoints::router(state).into())
}
