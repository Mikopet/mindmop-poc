mod graph;

use axum::{routing::get, Router};
use lazy_static::lazy_static;
use libsql::Database;
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_turso::Turso;

lazy_static! {
    static ref SEED: bool = std::env::var("SEED")
        .unwrap_or_default()
        .parse()
        .unwrap_or(false);
}

async fn hello_world() -> &'static str {
    "Hello, world!"
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
    let router = Router::new().route("/", get(hello_world));

    // Build DB connection
    let db = db.connect().unwrap();

    // Set up graph schema
    graph::schema::setup(&db).await?;

    // Boot Axum Server
    Ok(router.into())
}
