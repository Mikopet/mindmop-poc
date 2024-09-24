use libsql::{Connection, Transaction};
use shuttle_runtime::Error;
use std::fs::read_to_string;

pub async fn setup(db: &Connection) -> Result<bool, Error> {
    // Create transaction for schema drop->create->seed
    let t = db
        .transaction()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    println!("[schema] TRANSACTION START {{");

    // Do the deed
    let drop_count = command(&t, "DROP").await?;
    command(&t, "CREATE").await?;
    let seed_count = command(&t, "SEED").await?;

    // guardrail for not dropping valid data
    if drop_count <= seed_count {
        t.commit()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        println!("[schema] }} TRANSACTION COMMIT");

        return Ok(true);
    }

    t.rollback()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    println!("[schema] }} TRANSACTION ROLLBACK (due `drop_count` > `seed_count`)");

    Ok(false)
}

async fn command(t: &Transaction, s: &str) -> Result<u64, Error> {
    print!("[schema] {s:>10} | ");
    if !*crate::SEED && s != "CREATE" {
        println!("\u{23E9} skipped (due SEED=false)");
        return Ok(0);
    }

    let path = format!("src/graph/schema/sql/{s}.sql");
    let sql = read_to_string(&path).map_err(|e| {
        println!("\u{274C} failed to load schema file at `{path}`");
        Error::Io(e)
    })?;

    t.execute_batch(&sql)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

    let count = t.changes();
    let mut affected = String::from("");

    if s != "CREATE" {
        affected = format!("({count} rows affected)");
    }
    println!("\u{2705} completed {affected}");

    Ok(count)
}
