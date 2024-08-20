use serde::{Deserialize, Serialize};
use libsql::Database;
use std::sync::Arc;

pub async fn getsize(client: Arc<Database>) -> i64 {
    let conn = client.connect().unwrap();
    let mut rows = conn.query("SELECT COUNT(*) FROM pyhz", ()).await.unwrap();
    let Some(row) = rows.next().await.unwrap() else { return 0 };
    row.get::<i64>(0).unwrap()
}
