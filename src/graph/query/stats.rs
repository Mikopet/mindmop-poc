use libsql::Connection;
use serde::Serialize;

use super::{count_all, count_orphan};

#[derive(Serialize)]
pub struct Stats {
    node_count_total: u64,
    node_count_orphan: u64,
    difference: u64,
}

impl Stats {
    pub async fn get(db: &Connection) -> Result<Self, libsql::Error> {
        let total = count_all(db).await?;
        let orphan = count_orphan(db).await?;

        Ok(Self {
            node_count_total: total,
            node_count_orphan: orphan,
            difference: (total - orphan),
        })
    }
}
