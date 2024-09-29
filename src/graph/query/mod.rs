mod stats;

pub use stats::*;

use libsql::{Connection, Error};

async fn count_all(db: &Connection) -> Result<u64, Error> {
    db.query("SELECT COUNT(*) FROM graph", ())
        .await?
        .next()
        .await?
        .ok_or(libsql::Error::QueryReturnedNoRows)?
        .get(0)
}

async fn count_orphan(db: &Connection) -> Result<u64, Error> {
    db.query("SELECT COUNT(*) FROM graph WHERE target IS NULL", ())
        .await?
        .next()
        .await?
        .ok_or(libsql::Error::QueryReturnedNoRows)?
        .get(0)
}
